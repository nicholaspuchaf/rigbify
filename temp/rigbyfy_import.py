#!/usr/bin/env python3
"""Prepara e transfere bundles de músicas autorizadas para o Rigbyfy.

O script nunca procura músicas automaticamente. Para baixar algo, a URL HTTPS
precisa ser fornecida pelo usuário em sources.csv e --authorize é obrigatório.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import os
import re
import subprocess
import sys
import unicodedata
from dataclasses import asdict, dataclass
from datetime import UTC, datetime
from pathlib import Path
from typing import Any
from difflib import SequenceMatcher
from urllib.error import HTTPError, URLError
from urllib.parse import urlencode, urlparse
from urllib.request import urlopen


CSV_COLUMNS = {
    "Track URI", "Track Name", "Album Name", "Artist Name(s)", "Duration (ms)"
}
MANIFEST_NAME = "manifest.json"
YOUTUBE_API_URL = "https://www.googleapis.com/youtube/v3"
REJECTED_TITLE_MARKERS = {"cover", "live", "remix", "slowed", "nightcore", "karaoke"}


def load_dotenv(path: Path) -> None:
    """Carrega somente variáveis simples do .env local, sem sobrescrever o shell."""
    if not path.is_file():
        return
    for raw_line in path.read_text(encoding="utf-8").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        name, value = line.split("=", 1)
        name = name.strip()
        value = value.strip().strip("\"'")
        if name and value:
            os.environ.setdefault(name, value)


load_dotenv(Path(__file__).with_name(".env"))


@dataclass(frozen=True)
class Track:
    key: str
    spotify_uri: str
    title: str
    artist: str
    album: str
    duration_ms: int | None
    file: str | None = None
    source_url: str | None = None
    status: str = "missing_source"


def clean(value: str | None) -> str:
    return (value or "").strip()


def track_key(row: dict[str, str]) -> str:
    spotify_uri = clean(row.get("Track URI"))
    if spotify_uri:
        return spotify_uri.rsplit(":", 1)[-1]
    stable = "\x1f".join(
        clean(row.get(column)).casefold()
        for column in ("Track Name", "Artist Name(s)", "Album Name")
    )
    return f"local-{hashlib.sha256(stable.encode()).hexdigest()[:16]}"


def parse_duration(value: str | None) -> int | None:
    try:
        parsed = int(clean(value))
        return parsed if parsed > 0 else None
    except ValueError:
        return None


def playlist_name(path: Path) -> str:
    return re.sub(r"\s+", " ", path.stem.replace("_", " ")).strip()


def read_playlist(path: Path) -> list[Track]:
    with path.open("r", encoding="utf-8-sig", newline="") as handle:
        reader = csv.DictReader(handle)
        if not reader.fieldnames or not CSV_COLUMNS.issubset(set(reader.fieldnames)):
            raise ValueError("não possui o cabeçalho de exportação de playlist esperado")
        tracks: list[Track] = []
        for row in reader:
            title = clean(row.get("Track Name"))
            artist = clean(row.get("Artist Name(s)"))
            if not title or not artist:
                continue
            tracks.append(
                Track(
                    key=track_key(row),
                    spotify_uri=clean(row.get("Track URI")),
                    title=title,
                    artist=artist,
                    album=clean(row.get("Album Name")),
                    duration_ms=parse_duration(row.get("Duration (ms)")),
                )
            )
    return tracks


def write_json(path: Path, value: Any) -> None:
    temporary = path.with_suffix(f"{path.suffix}.tmp")
    temporary.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    temporary.replace(path)


def prepare(input_dir: Path, bundle_dir: Path) -> None:
    csv_files = sorted(path for path in input_dir.glob("*.csv") if path.name != "sources.csv")
    if not csv_files:
        raise ValueError(f"nenhum CSV encontrado em {input_dir}")
    playlists: list[dict[str, Any]] = []
    unique: dict[str, Track] = {}
    for csv_file in csv_files:
        try:
            tracks = read_playlist(csv_file)
        except ValueError as error:
            print(f"ignorando {csv_file.name}: {error}", file=sys.stderr)
            continue
        if not tracks:
            continue
        for track in tracks:
            unique.setdefault(track.key, track)
        playlists.append(
            {
                "id": f"csv-{hashlib.sha256(csv_file.name.encode()).hexdigest()[:16]}",
                "name": playlist_name(csv_file),
                "sourceCsv": csv_file.name,
                "trackKeys": [track.key for track in tracks],
            }
        )
    bundle_dir.mkdir(parents=True, exist_ok=True)
    (bundle_dir / "audio").mkdir(exist_ok=True)
    previous_sources = (
        read_sources(bundle_dir / "sources.csv")
        if (bundle_dir / "sources.csv").is_file()
        else {}
    )
    manifest = {
        "schemaVersion": 1,
        "generatedAt": datetime.now(UTC).isoformat(),
        "playlists": playlists,
        "tracks": {key: asdict(track) for key, track in unique.items()},
    }
    write_json(bundle_dir / MANIFEST_NAME, manifest)
    sources_path = bundle_dir / "sources.csv"
    with sources_path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(
            handle,
            fieldnames=["spotify_uri", "track_key", "title", "artist", "album", "source_url"],
        )
        writer.writeheader()
        for track in sorted(unique.values(), key=lambda item: (item.artist.casefold(), item.title.casefold())):
            writer.writerow(
                {
                    "spotify_uri": track.spotify_uri,
                    "track_key": track.key,
                    "title": track.title,
                    "artist": track.artist,
                    "album": track.album,
                    "source_url": previous_sources.get(track.key, ""),
                }
            )
    print(f"Bundle criado: {bundle_dir}")
    print(f"Playlists: {len(playlists)} | faixas únicas: {len(unique)}")
    print(f"Preencha URLs autorizadas em: {sources_path}")


def read_manifest(bundle_dir: Path) -> dict[str, Any]:
    path = bundle_dir / MANIFEST_NAME
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as error:
        raise ValueError(f"manifest ausente: execute prepare antes ({path})") from error
    if value.get("schemaVersion") != 1 or not isinstance(value.get("tracks"), dict):
        raise ValueError("manifest inválido ou incompatível")
    return value


def read_sources(path: Path) -> dict[str, str]:
    with path.open("r", encoding="utf-8-sig", newline="") as handle:
        reader = csv.DictReader(handle)
        required = {"track_key", "source_url"}
        if not reader.fieldnames or not required.issubset(set(reader.fieldnames)):
            raise ValueError("sources.csv deve conter track_key e source_url")
        result = {clean(row.get("track_key")): clean(row.get("source_url")) for row in reader}
    return {key: url for key, url in result.items() if url}


def allowed_source_url(value: str) -> bool:
    parsed = urlparse(value)
    return parsed.scheme == "https" and bool(parsed.netloc)


def normalized(value: str | None) -> str:
    decomposed = unicodedata.normalize("NFKD", clean(value))
    without_accents = "".join(character for character in decomposed if not unicodedata.combining(character))
    return re.sub(r"[^a-z0-9]+", " ", without_accents.casefold()).strip()


def duration_seconds(value: str | None) -> int | None:
    match = re.fullmatch(r"PT(?:(\d+)H)?(?:(\d+)M)?(?:(\d+)S)?", clean(value))
    if not match:
        return None
    hours, minutes, seconds = (int(part or 0) for part in match.groups())
    return hours * 3600 + minutes * 60 + seconds


def candidate_score(track: dict[str, Any], candidate: dict[str, Any]) -> tuple[int, str]:
    title = normalized(track.get("title"))
    artist = normalized(track.get("artist"))
    candidate_title = normalized(candidate.get("title"))
    candidate_channel = normalized(candidate.get("channel"))
    title_score = SequenceMatcher(None, title, candidate_title).ratio() * 55
    artist_score = 25 if artist and artist in f"{candidate_title} {candidate_channel}" else 0
    duration_score = 0
    expected_duration = track.get("duration_ms")
    actual_duration = candidate.get("duration_seconds")
    if isinstance(expected_duration, int) and isinstance(actual_duration, int):
        difference = abs(expected_duration // 1000 - actual_duration)
        duration_score = 20 if difference <= 5 else 12 if difference <= 15 else -20
    markers = [marker for marker in REJECTED_TITLE_MARKERS if marker in candidate_title]
    score = max(0, min(100, round(title_score + artist_score + duration_score - len(markers) * 30)))
    checks = [f"título:{round(title_score / 55 * 100)}%"]
    if artist_score:
        checks.append("artista/canal")
    if duration_score:
        checks.append(f"duração:{duration_score}")
    if markers:
        checks.append("revisar:" + "/".join(markers))
    return score, "; ".join(checks)


def youtube_api_get(resource: str, params: dict[str, str], api_key: str) -> dict[str, Any]:
    query = urlencode({**params, "key": api_key})
    request_url = f"{YOUTUBE_API_URL}/{resource}?{query}"
    try:
        with urlopen(request_url, timeout=20) as response:  # noqa: S310 - endpoint constante da API oficial
            return json.loads(response.read().decode("utf-8"))
    except HTTPError as error:
        detail = error.read().decode("utf-8", errors="replace")[:500]
        raise RuntimeError(f"YouTube Data API respondeu HTTP {error.code}: {detail}") from error
    except URLError as error:
        raise RuntimeError(f"não foi possível alcançar a YouTube Data API: {error.reason}") from error


def youtube_candidates(track: dict[str, Any], api_key: str, candidate_limit: int) -> list[dict[str, Any]]:
    query = " ".join(filter(None, [clean(track.get("artist")), clean(track.get("title")), clean(track.get("album"))]))
    search = youtube_api_get(
        "search",
        {"part": "snippet", "q": query, "type": "video", "maxResults": str(candidate_limit)},
        api_key,
    )
    identifiers = [item.get("id", {}).get("videoId") for item in search.get("items", [])]
    video_ids = [identifier for identifier in identifiers if isinstance(identifier, str)]
    if not video_ids:
        return []
    details = youtube_api_get(
        "videos",
        {"part": "snippet,contentDetails", "id": ",".join(video_ids)},
        api_key,
    )
    found = {item.get("id"): item for item in details.get("items", [])}
    candidates: list[dict[str, Any]] = []
    for video_id in video_ids:
        video = found.get(video_id)
        if not isinstance(video, dict):
            continue
        snippet = video.get("snippet", {})
        content_details = video.get("contentDetails", {})
        candidate = {
            "video_id": video_id,
            "url": f"https://www.youtube.com/watch?v={video_id}",
            "title": clean(snippet.get("title")),
            "channel": clean(snippet.get("channelTitle")),
            "duration_seconds": duration_seconds(content_details.get("duration")),
        }
        candidate["score"], candidate["checks"] = candidate_score(track, candidate)
        candidates.append(candidate)
    return sorted(candidates, key=lambda item: item["score"], reverse=True)


def selected_playlist(manifest: dict[str, Any], name: str) -> dict[str, Any]:
    wanted = clean(name).casefold()
    matches = [playlist for playlist in manifest["playlists"] if clean(playlist["name"]).casefold() == wanted]
    if not matches:
        available = ", ".join(playlist["name"] for playlist in manifest["playlists"])
        raise ValueError(f"playlist não encontrada: {name}. Disponíveis: {available}")
    return matches[0]


def require_playlist_authorization(playlist_name: str, authorization: str) -> None:
    if clean(authorization) != clean(playlist_name):
        raise ValueError("a confirmação deve repetir exatamente o nome da playlist escolhida")


def review_path(bundle_dir: Path, playlist: dict[str, Any]) -> Path:
    return bundle_dir / f"review-{playlist['id']}.csv"


def write_sources(bundle_dir: Path, manifest: dict[str, Any], sources: dict[str, str]) -> Path:
    sources_path = bundle_dir / "sources.csv"
    with sources_path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=["spotify_uri", "track_key", "title", "artist", "album", "source_url"])
        writer.writeheader()
        for track in sorted(manifest["tracks"].values(), key=lambda item: (item["artist"].casefold(), item["title"].casefold())):
            writer.writerow(
                {
                    "spotify_uri": track["spotify_uri"], "track_key": track["key"], "title": track["title"],
                    "artist": track["artist"], "album": track["album"], "source_url": sources.get(track["key"], ""),
                }
            )
    return sources_path


def choose_candidate(track: dict[str, Any], candidates: list[dict[str, Any]]) -> dict[str, Any] | None:
    print("\n" + "=" * 72)
    print(f"Faixa: {track['artist']} — {track['title']}")
    if track.get("album"):
        print(f"Álbum: {track['album']}")
    if track.get("duration_ms"):
        print(f"Duração esperada: {track['duration_ms'] // 1000}s")
    if not candidates:
        print("Nenhum candidato encontrado. Pressione Enter para continuar.")
        input()
        return None
    for index, candidate in enumerate(candidates, start=1):
        print(f"\n[{index}] {candidate['title']}")
        print(f"    Canal: {candidate['channel'] or 'não informado'}")
        print(f"    Duração: {candidate['duration_seconds'] if candidate['duration_seconds'] is not None else 'não informada'}s")
        print(f"    Pontuação: {candidate['score']}/100 | {candidate['checks']}")
        print(f"    URL: {candidate['url']}")
    while True:
        answer = input("Escolha 1–{}; s para pular; q para encerrar: ".format(len(candidates))).strip().casefold()
        if answer == "s":
            return None
        if answer == "q":
            raise KeyboardInterrupt
        if answer.isdigit() and 1 <= int(answer) <= len(candidates):
            return candidates[int(answer) - 1]
        print("Opção inválida.")


def discover(
    bundle_dir: Path,
    playlist_name: str,
    authorization: str,
    api_key: str,
    limit: int | None,
    candidate_limit: int,
    interactive: bool,
    auto_select: bool,
) -> None:
    require_playlist_authorization(playlist_name, authorization)
    manifest = read_manifest(bundle_dir)
    playlist = selected_playlist(manifest, playlist_name)
    sources = read_sources(bundle_dir / "sources.csv")
    tracks = [
        manifest["tracks"][key]
        for key in playlist["trackKeys"]
        if not sources.get(key)
    ]
    if limit is not None:
        tracks = tracks[:limit]
    output = review_path(bundle_dir, playlist)
    fieldnames = [
        "track_key", "spotify_uri", "source_title", "source_artist", "source_album", "source_duration_seconds",
        "candidate_rank", "candidate_score", "candidate_checks", "candidate_title", "candidate_channel",
        "candidate_duration_seconds", "candidate_url", "approved", "approved_url",
    ]
    print(f"Playlist autorizada para descoberta: {playlist['name']} ({len(tracks)} faixas)")
    if interactive and auto_select:
        raise ValueError("use apenas uma das opções --interactive ou --auto-select")
    if interactive and not sys.stdin.isatty():
        raise RuntimeError("--interactive exige um terminal com entrada interativa")
    selected_count = 0
    with output.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        for index, track in enumerate(tracks, start=1):
            print(f"[{index}/{len(tracks)}] consultando {track['artist']} — {track['title']}")
            candidates = youtube_candidates(track, api_key, candidate_limit)
            selected = choose_candidate(track, candidates) if interactive else candidates[0] if auto_select and candidates else None
            if selected:
                if auto_select:
                    print(f"  Selecionado automaticamente: {selected['title']} | {selected['url']} | pontuação {selected['score']}/100")
                sources[track["key"]] = selected["url"]
                write_sources(bundle_dir, manifest, sources)
                selected_count += 1
            if not candidates:
                writer.writerow(
                    {
                        "track_key": track["key"], "spotify_uri": track["spotify_uri"],
                        "source_title": track["title"], "source_artist": track["artist"],
                        "source_album": track["album"],
                        "source_duration_seconds": (track.get("duration_ms") or 0) // 1000,
                    }
                )
                continue
            for rank, candidate in enumerate(candidates, start=1):
                writer.writerow(
                    {
                        "track_key": track["key"], "spotify_uri": track["spotify_uri"],
                        "source_title": track["title"], "source_artist": track["artist"],
                        "source_album": track["album"],
                        "source_duration_seconds": (track.get("duration_ms") or 0) // 1000,
                        "candidate_rank": rank, "candidate_score": candidate["score"],
                        "candidate_checks": candidate["checks"], "candidate_title": candidate["title"],
                        "candidate_channel": candidate["channel"],
                        "candidate_duration_seconds": candidate["duration_seconds"], "candidate_url": candidate["url"],
                        "approved": "sim" if selected and candidate["url"] == selected["url"] else "",
                        "approved_url": selected["url"] if selected and candidate["url"] == selected["url"] else "",
                    }
                )
    print(f"Revise e aprove manualmente as linhas em: {output}")
    if interactive:
        print(f"{selected_count} URL(s) escolhida(s) já foram aplicadas em: {bundle_dir / 'sources.csv'}")
    else:
        print("Use approved=sim e copie o candidato escolhido para approved_url; depois execute apply-review.")


def apply_review(bundle_dir: Path, review: Path) -> None:
    sources_path = bundle_dir / "sources.csv"
    sources = read_sources(sources_path)
    with review.open("r", encoding="utf-8-sig", newline="") as handle:
        reader = csv.DictReader(handle)
        required = {"track_key", "approved", "approved_url"}
        if not reader.fieldnames or not required.issubset(set(reader.fieldnames)):
            raise ValueError("review.csv deve conter track_key, approved e approved_url")
        approved = [row for row in reader if normalized(row.get("approved")) in {"sim", "yes", "true", "1"}]
    invalid = [row.get("track_key", "") for row in approved if not allowed_source_url(clean(row.get("approved_url")))]
    if invalid:
        raise ValueError(f"URLs aprovadas inválidas ou não HTTPS: {', '.join(invalid[:5])}")
    for row in approved:
        sources[clean(row.get("track_key"))] = clean(row.get("approved_url"))
    manifest = read_manifest(bundle_dir)
    write_sources(bundle_dir, manifest, sources)
    print(f"{len(approved)} URL(s) aprovada(s) aplicada(s) em: {sources_path}")


def download(
    bundle_dir: Path,
    sources_path: Path,
    playlist_name: str,
    authorization: str,
    limit: int | None,
    retry: bool,
) -> None:
    require_playlist_authorization(playlist_name, authorization)
    manifest = read_manifest(bundle_dir)
    playlist = selected_playlist(manifest, playlist_name)
    sources = read_sources(sources_path)
    audio_dir = bundle_dir / "audio"
    audio_dir.mkdir(exist_ok=True)
    candidates = []
    for key in playlist["trackKeys"]:
        track = manifest["tracks"][key]
        url = sources.get(key)
        if not url or (track.get("status") == "downloaded" and not retry):
            continue
        candidates.append((key, url))
    if limit is not None:
        candidates = candidates[:limit]
    if not candidates:
        print(f"Nenhuma faixa pendente com URL preenchida em: {playlist['name']}")
        return
    try:
        import yt_dlp
    except ImportError as error:
        raise RuntimeError("yt-dlp indisponível; execute este script com uv run") from error
    print(f"Playlist autorizada: {playlist['name']} ({len(candidates)} faixas pendentes)")
    for index, (key, source_url) in enumerate(candidates, start=1):
        if not allowed_source_url(source_url):
            print(f"[{index}/{len(candidates)}] ignorada URL não HTTPS para {key}", file=sys.stderr)
            continue
        output_template = str(audio_dir / f"{key}.%(ext)s")
        options = {
            "format": "bestaudio[ext=m4a]/bestaudio/best",
            "outtmpl": output_template,
            "noplaylist": True,
            "no_warnings": False,
            "quiet": False,
            "writethumbnail": False,
            "addmetadata": True,
        }
        print(f"[{index}/{len(candidates)}] baixando {manifest['tracks'][key]['artist']} — {manifest['tracks'][key]['title']}")
        try:
            with yt_dlp.YoutubeDL(options) as ydl:
                info = ydl.extract_info(source_url, download=True)
                filename = Path(ydl.prepare_filename(info))
            matches = sorted(audio_dir.glob(f"{key}.*"), key=lambda path: path.stat().st_mtime, reverse=True)
            if not matches:
                raise RuntimeError(f"arquivo final ausente (último nome previsto: {filename.name})")
            track = manifest["tracks"][key]
            track["file"] = str(matches[0].relative_to(bundle_dir))
            track["source_url"] = source_url
            track["status"] = "downloaded"
            write_json(bundle_dir / MANIFEST_NAME, manifest)
        except Exception as error:  # yt-dlp expõe diferentes tipos por versão
            manifest["tracks"][key]["status"] = "failed"
            manifest["tracks"][key]["source_url"] = source_url
            write_json(bundle_dir / MANIFEST_NAME, manifest)
            print(f"falhou para {key}: {error}", file=sys.stderr)


def verify(bundle_dir: Path) -> None:
    manifest = read_manifest(bundle_dir)
    downloaded = missing = 0
    for track in manifest["tracks"].values():
        file_value = track.get("file")
        if file_value and (bundle_dir / file_value).is_file():
            downloaded += 1
        else:
            missing += 1
    print(f"Arquivos disponíveis: {downloaded} | pendentes/falhos: {missing}")


def push(bundle_dir: Path, serial: str | None) -> None:
    read_manifest(bundle_dir)
    command = ["adb"]
    if serial:
        command.extend(["-s", serial])
    command.extend(["push", str(bundle_dir), "/sdcard/Download/"])
    subprocess.run(command, check=True)
    print("Bundle enviado para /sdcard/Download/Rigbyfy-import")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    prepare_parser = commands.add_parser("prepare", help="gera manifest.json e sources.csv")
    prepare_parser.add_argument("--input-dir", type=Path, default=Path(__file__).parent)
    prepare_parser.add_argument("--output-dir", type=Path, default=Path(__file__).parent / "Rigbyfy-import")
    download_parser = commands.add_parser("download", help="baixa uma playlist autorizada de sources.csv")
    download_parser.add_argument("--bundle-dir", type=Path, default=Path(__file__).parent / "Rigbyfy-import")
    download_parser.add_argument("--sources", type=Path)
    download_parser.add_argument("--playlist", required=True, help="nome da playlist a baixar")
    download_parser.add_argument("--authorize", required=True, help="repita exatamente o nome da playlist como confirmação")
    download_parser.add_argument("--limit", type=int)
    download_parser.add_argument("--retry", action="store_true", help="tenta novamente faixas já marcadas como baixadas")
    discover_parser = commands.add_parser("discover", help="gera candidatos do YouTube para revisão manual")
    discover_parser.add_argument("--bundle-dir", type=Path, default=Path(__file__).parent / "Rigbyfy-import")
    discover_parser.add_argument("--playlist", required=True, help="nome da playlist a consultar")
    discover_parser.add_argument("--authorize", required=True, help="repita exatamente o nome da playlist como confirmação")
    discover_parser.add_argument("--api-key", default=os.environ.get("YOUTUBE_API_KEY"), help="chave da YouTube Data API; prefira YOUTUBE_API_KEY")
    discover_parser.add_argument("--limit", type=int, default=10, help="máximo de faixas nesta execução (padrão: 10)")
    discover_parser.add_argument("--candidate-limit", type=int, default=3, choices=range(1, 6), help="candidatos por faixa (1 a 5)")
    discover_parser.add_argument("--interactive", action="store_true", help="escolhe candidatos no terminal e atualiza sources.csv")
    discover_parser.add_argument("--auto-select", action="store_true", help="aplica automaticamente o candidato de maior pontuação")
    review_parser = commands.add_parser("apply-review", help="aplica URLs aprovadas de um review CSV ao sources.csv")
    review_parser.add_argument("--bundle-dir", type=Path, default=Path(__file__).parent / "Rigbyfy-import")
    review_parser.add_argument("--review", type=Path, required=True)
    verify_parser = commands.add_parser("verify", help="confere arquivos referenciados no manifest")
    verify_parser.add_argument("--bundle-dir", type=Path, default=Path(__file__).parent / "Rigbyfy-import")
    push_parser = commands.add_parser("push", help="envia bundle para Downloads via adb")
    push_parser.add_argument("--bundle-dir", type=Path, default=Path(__file__).parent / "Rigbyfy-import")
    push_parser.add_argument("--serial")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        if args.command == "prepare":
            prepare(args.input_dir, args.output_dir)
        elif args.command == "download":
            download(
                args.bundle_dir,
                args.sources or args.bundle_dir / "sources.csv",
                args.playlist,
                args.authorize,
                args.limit,
                args.retry,
            )
        elif args.command == "discover":
            if not args.api_key:
                raise ValueError("informe uma chave via YOUTUBE_API_KEY ou --api-key")
            discover(
                args.bundle_dir,
                args.playlist,
                args.authorize,
                args.api_key,
                args.limit,
                args.candidate_limit,
                args.interactive,
                args.auto_select,
            )
        elif args.command == "apply-review":
            apply_review(args.bundle_dir, args.review)
        elif args.command == "verify":
            verify(args.bundle_dir)
        else:
            push(args.bundle_dir, args.serial)
        return 0
    except (ValueError, RuntimeError, subprocess.CalledProcessError) as error:
        print(f"erro: {error}", file=sys.stderr)
        return 1
    except KeyboardInterrupt:
        print("descoberta interrompida; escolhas anteriores foram preservadas", file=sys.stderr)
        return 130


if __name__ == "__main__":
    raise SystemExit(main())

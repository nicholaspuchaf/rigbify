"""Testes locais da ferramenta de preparação de bundles."""

from __future__ import annotations

import csv
import io
import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import MagicMock, patch
from urllib.error import HTTPError


SCRIPT = Path(__file__).with_name("rigbyfy_import.py")
SPEC = importlib.util.spec_from_file_location("rigbyfy_import", SCRIPT)
assert SPEC and SPEC.loader
IMPORTER = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = IMPORTER
SPEC.loader.exec_module(IMPORTER)


class PrepareBundleTests(unittest.TestCase):
    def write_playlist(self, directory: Path, name: str, rows: list[dict[str, str]]) -> None:
        with (directory / name).open("w", encoding="utf-8", newline="") as handle:
            writer = csv.DictWriter(handle, fieldnames=sorted(IMPORTER.CSV_COLUMNS))
            writer.writeheader()
            writer.writerows(rows)

    def test_prepare_ignores_invalid_csv_and_preserves_reviewed_source(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            bundle = root / "Rigbyfy-import"
            row = {
                "Track URI": "spotify:track:track-one",
                "Track Name": "A faixa",
                "Album Name": "O álbum",
                "Artist Name(s)": "A artista",
                "Duration (ms)": "123000",
            }
            self.write_playlist(root, "Minha_playlist.csv", [row])
            (root / "invalido.csv").write_text("coluna_incorreta\nvalor\n", encoding="utf-8")

            IMPORTER.prepare(root, bundle)
            sources = bundle / "sources.csv"
            with sources.open("r", encoding="utf-8", newline="") as handle:
                reader = csv.DictReader(handle)
                rows = list(reader)
                fieldnames = reader.fieldnames
            assert fieldnames
            rows[0]["source_url"] = "https://example.com/media"
            with sources.open("w", encoding="utf-8", newline="") as handle:
                writer = csv.DictWriter(handle, fieldnames=fieldnames)
                writer.writeheader()
                writer.writerows(rows)

            IMPORTER.prepare(root, bundle)
            self.assertIn("https://example.com/media", sources.read_text(encoding="utf-8"))

    def test_playlist_authorization_requires_exact_name(self) -> None:
        with self.assertRaisesRegex(ValueError, "confirmação"):
            IMPORTER.download(Path("."), Path("sources.csv"), "Rock brabo", "rock brabo", None, False, False)

    def test_artwork_only_candidates_never_include_missing_audio(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            bundle = Path(temporary)
            (bundle / "audio").mkdir()
            (bundle / "audio" / "downloaded.m4a").write_bytes(b"audio")
            playlist = {"trackKeys": ["downloaded", "missing", "covered"]}
            manifest = {"tracks": {
                "downloaded": {"status": "downloaded", "file": "audio/downloaded.m4a"},
                "missing": {"status": "missing_source", "file": None},
                "covered": {"status": "downloaded", "file": "audio/downloaded.m4a", "artwork": "artwork/covered.jpg"},
            }}
            artwork = bundle / "artwork"
            artwork.mkdir()
            (artwork / "covered.jpg").write_bytes(b"cover")
            candidates = IMPORTER.download_candidates(
                bundle,
                playlist,
                manifest,
                {"downloaded": "https://example.com/one", "missing": "https://example.com/two", "covered": "https://example.com/three"},
                retry=False,
                artwork_only=True,
            )
        self.assertEqual(candidates, [("downloaded", "https://example.com/one", True)])

    def test_candidate_score_penalizes_live_and_wrong_duration(self) -> None:
        score, checks = IMPORTER.candidate_score(
            {"title": "Minha música", "artist": "Banda", "duration_ms": 180_000},
            {"title": "Banda - Minha música (Live)", "channel": "Canal oficial", "duration_seconds": 300},
        )
        self.assertLess(score, 60)
        self.assertIn("revisar:live", checks)

    def test_configured_youtube_keys_orders_numbered_environment_variables_without_duplicates(self) -> None:
        with patch.dict(
            IMPORTER.os.environ,
            {"YOUTUBE_API_KEY": "first", "YOUTUBE_API_KEY2": "third", "YOUTUBE_API_KEY1": "second", "YOUTUBE_API_KEY3": "second"},
            clear=True,
        ):
            self.assertEqual(IMPORTER.configured_youtube_keys(), ["first", "second", "third"])

    def test_quota_error_is_identified_without_exposing_key_material(self) -> None:
        error = HTTPError("https://example.com", 403, "forbidden", {}, None)
        try:
            self.assertTrue(IMPORTER.quota_exhausted(error, '{"error":{"errors":[{"reason":"quotaExceeded"}]}}'))
            self.assertFalse(IMPORTER.quota_exhausted(error, '{"error":{"errors":[{"reason":"forbidden"}]}}'))
        finally:
            error.close()

    def test_youtube_request_retries_with_next_key_after_quota_error(self) -> None:
        quota_error = HTTPError(
            "https://example.com",
            403,
            "forbidden",
            {},
            io.BytesIO(b'{"error":{"errors":[{"reason":"quotaExceeded"}]}}'),
        )
        response = MagicMock()
        response.read.return_value = b'{"items": []}'
        context = MagicMock()
        context.__enter__.return_value = response
        with patch.object(IMPORTER, "urlopen", side_effect=[quota_error, context]) as mocked_urlopen:
            result = IMPORTER.youtube_api_get("search", {"part": "snippet"}, IMPORTER.YoutubeKeyPool(["first", "second"]))
        self.assertEqual(result, {"items": []})
        self.assertIn("key=second", mocked_urlopen.call_args_list[1].args[0])

    def test_apply_review_only_applies_explicit_https_approval(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            bundle = root / "Rigbyfy-import"
            row = {
                "Track URI": "spotify:track:track-one",
                "Track Name": "A faixa",
                "Album Name": "O álbum",
                "Artist Name(s)": "A artista",
                "Duration (ms)": "123000",
            }
            self.write_playlist(root, "Minha_playlist.csv", [row])
            IMPORTER.prepare(root, bundle)
            review = bundle / "review.csv"
            review.write_text(
                "track_key,approved,approved_url\ntrack-one,sim,https://example.com/video\n",
                encoding="utf-8",
            )

            IMPORTER.apply_review(bundle, review)
            with (bundle / "sources.csv").open("r", encoding="utf-8", newline="") as handle:
                source_rows = list(csv.DictReader(handle))
            self.assertEqual(source_rows[0]["source_url"], "https://example.com/video")

    def test_terminal_choice_returns_the_selected_candidate(self) -> None:
        candidate = {
            "url": "https://example.com/video", "title": "Vídeo", "channel": "Canal",
            "duration_seconds": 120, "score": 90, "checks": "título:100%",
        }
        with patch("builtins.input", return_value="1"):
            selected = IMPORTER.choose_candidate({"artist": "Artista", "title": "Faixa"}, [candidate])
        self.assertEqual(selected, candidate)

    def test_download_artwork_writes_supported_thumbnail(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            response = MagicMock()
            response.read.return_value = b"thumbnail"
            response.headers.get_content_type.return_value = "image/jpeg"
            context = MagicMock()
            context.__enter__.return_value = response
            with patch.object(IMPORTER, "urlopen", return_value=context):
                artwork = IMPORTER.download_artwork(
                    "https://i.ytimg.com/vi/example/maxresdefault.jpg",
                    Path(temporary),
                    "track-one",
                )
            self.assertEqual(artwork, Path(temporary) / "track-one.jpg")
            self.assertEqual(artwork.read_bytes(), b"thumbnail")

    def test_local_cookie_file_requires_a_nonempty_local_file(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            self.assertIsNone(IMPORTER.local_cookie_file(directory))
            cookie_file = directory / IMPORTER.COOKIE_FILE_NAME
            cookie_file.write_text("# Netscape HTTP Cookie File\\n", encoding="utf-8")
            self.assertEqual(IMPORTER.local_cookie_file(directory), cookie_file)

    def test_youtube_download_options_enable_node_and_keep_cookie_local(self) -> None:
        cookie_file = Path(".cookie.txt")
        options = IMPORTER.youtube_download_options("audio/%(id)s.%(ext)s", cookie_file)
        self.assertEqual(options["js_runtimes"], {"node": {"path": None}})
        self.assertEqual(options["remote_components"], ["ejs:github"])
        self.assertEqual(options["cookiefile"], str(cookie_file))


if __name__ == "__main__":
    unittest.main()

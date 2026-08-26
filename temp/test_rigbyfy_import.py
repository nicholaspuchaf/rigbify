"""Testes locais da ferramenta de preparação de bundles."""

from __future__ import annotations

import csv
import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch


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
            IMPORTER.download(Path("."), Path("sources.csv"), "Rock brabo", "rock brabo", None, False)

    def test_candidate_score_penalizes_live_and_wrong_duration(self) -> None:
        score, checks = IMPORTER.candidate_score(
            {"title": "Minha música", "artist": "Banda", "duration_ms": 180_000},
            {"title": "Banda - Minha música (Live)", "channel": "Canal oficial", "duration_seconds": 300},
        )
        self.assertLess(score, 60)
        self.assertIn("revisar:live", checks)

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


if __name__ == "__main__":
    unittest.main()

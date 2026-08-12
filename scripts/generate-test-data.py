#!/usr/bin/env python3

import argparse
import csv
import random
from pathlib import Path


def generate(output: Path, items: int, annotators: int, labels: list[str]) -> None:
    output.parent.mkdir(parents=True, exist_ok=True)

    with output.open("w", newline="", encoding="utf-8") as file:
        writer = csv.writer(file)
        writer.writerow(["item_id", "annotator_id", "label"])

        for item_index in range(items):
            item_id = f"item_{item_index:08d}"

            for annotator_index in range(annotators):
                annotator_id = f"worker_{annotator_index:04d}"
                label = random.choice(labels)

                writer.writerow([
                    item_id,
                    annotator_id,
                    label,
                ])


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Generate synthetic ACE annotation data."
    )

    parser.add_argument(
        "--items",
        type=int,
        default=1000,
    )

    parser.add_argument(
        "--annotators",
        type=int,
        default=5,
    )

    parser.add_argument(
        "--output",
        type=Path,
        default=Path("data/generated/annotations.csv"),
    )

    args = parser.parse_args()

    labels = ["cat", "dog", "bird", "fish"]

    generate(
        output=args.output,
        items=args.items,
        annotators=args.annotators,
        labels=labels,
    )

    print(f"Generated dataset: {args.output}")
    print(f"Items: {args.items}")
    print(f"Annotators: {args.annotators}")


if __name__ == "__main__":
    main()
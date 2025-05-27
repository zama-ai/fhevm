#!/bin/env python
"""Check links to local files."""

import json
import re
import sys
import tempfile
from pathlib import Path
from typing import List, Optional, Union

import linkcheckmd as lc

# A regex that matches [foo (bar)](my_link) and returns the my_link
# used to find all links made in our markdown files.
MARKDOWN_LINK_REGEX = [re.compile(r"\[[^\]]*\]\(([^\)]*)\)"), re.compile(r"href=\"[^\"]*\"")]


# pylint: disable-next=too-many-branches
def check_content_for_dead_links(
    content: str, file_path: Path, cell_id: Optional[int] = None
) -> List[str]:
    """Check the content of a markdown file for dead links.

    This checks a markdown file for dead-links to local files.

    Args:
        content (str): The content of the file.
        file_path (Path): The path to the file.
        cell_id (Optional[int]): the id of the notebook cell

    Returns:
        List[str]: a list of errors (dead-links) found.
    """
    errors: List[str] = []
    links = []
    for regex in MARKDOWN_LINK_REGEX:
        links_found = regex.findall(content)
        for link in links_found:
            link = link.replace(r"\_", "_")  # for gitbook
            if "href=" in link:  # for html links
                link = link.replace('href="', "")  # remove href=""
                link = link[0:-1]  # remove last "
            links.append(link)

    for link in links:
        link = link.strip()
        if link.startswith("http"):
            # This means this is a reference to a website
            continue
        if link.startswith("<http"):
            # This means this is a reference to a website
            continue
        if link.startswith("#"):
            # This means this is a reference to a header
            continue
        if link.startswith("mailto:"):
            # This means this is a reference to an email
            continue
        if "#" in link:
            # This means this is a reference to a file with header
            link = link.split("#")[0]

        link_path = file_path.parent / link
        ext = link_path.suffix
        link_path_no_ext = link_path.parent / link_path.stem

        file_path_display = str(file_path)
        if cell_id:
            file_path_display += f"/cell:{cell_id}"

        if ext == ".html":
            rst_alternative = link_path_no_ext.with_suffix(".rst")
            if not link_path.exists() and not rst_alternative.exists():
                errors.append(
                    f"{file_path_display} contains a link to {link_path} "
                    f"could not find either files:\n{link_path}\n{rst_alternative}"
                )
            continue

        if not link_path.exists():
            errors.append(
                f"{file_path_display} contains a link to"
                f" file '{link_path.resolve()}' that can't be found"
            )
    return errors


def is_relative_to(path: Path, other_path: Union[str, Path]) -> bool:
    """Implementation of is_relative_to

    is_relative_to is not available until python 3.9
    https://docs.python.org/3.9/library/pathlib.html#pathlib.PurePath.is_relative_to

    Args:
        path (Path): some path.
        other_path (str or Path): some other path.

    Returns:
        True if some path is relative to another.
    """
    try:
        path.relative_to(other_path)
        return True
    except ValueError:
        return False


def main():
    """Main function

    Check all files (except those that match a pattern in .gitignore) for
    dead links to local files.
    """
    root = Path("./")
    errors: List[str] = []

    gitignore_file = root / ".gitignore"
    if gitignore_file.exists():
        with gitignore_file.open(encoding="UTF-8") as file:
            ignores = file.read().split("\n")
            ignores = [elt for elt in map(lambda elt: elt.split("#")[0].strip(), ignores) if elt]

    # FIXME: do we want to start from .
    root = Path("./docs")

    for path in root.glob("**/*"):
        if (
            path.is_file()
            and path.suffix == ".md"
            and not any(is_relative_to(path, ignore) for ignore in ignores)
        ):
            print(f"checking {path}")
            with path.open() as file:
                file_content = file.read()
            errors += check_content_for_dead_links(file_content, path)

        if (
            path.is_file()
            and path.suffix == ".ipynb"
            and not any(is_relative_to(path, ignore) for ignore in ignores)
        ):
            print(f"checking {path}")
            with path.open() as file:
                nb_structure = json.load(file)
                if "cells" not in nb_structure:
                    print(f"Invalid notebook, skipping {path}")
                    continue
                cell_id = 0
                for cell in nb_structure["cells"]:
                    if cell["cell_type"] != "markdown":
                        cell_id += 1
                        continue

                    markdown_cell = "".join(cell["source"])
                    errors += check_content_for_dead_links(markdown_cell, path, cell_id)
                    cell_id += 1

                    with tempfile.NamedTemporaryFile(
                        delete=False, mode="wt", encoding="utf-8"
                    ) as fptr:
                        fptr.write(markdown_cell)
                        fptr.close()
                        bad = lc.check_links(fptr.name, ext=".*")
                        if bad:
                            for err_link in bad:
                                # Skip links to CML internal issues
                                if "zama-ai/concrete-ml-internal" in err_link[1]:
                                    continue

                                errors.append(
                                    f"{path}/cell:{cell_id} contains "
                                    f"a link to file '{err_link[1]}' that can't be found"
                                )

    if errors:
        errors.append(f"Number of errors: {len(errors)}")
        sys.exit("\n".join(errors))


if __name__ == "__main__":
    main()

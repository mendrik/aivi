#!/usr/bin/env python3
import sys
import os
import re

def fix_links(html_file):
    with open(html_file, 'r', encoding='utf-8') as f:
        content = f.read()

    # Match href="path/to/file.md" or href="file.md"
    # We want to convert them to href="#slug" where slug is path-to-file
    def link_repl(match):
        path = match.group(1)
        # Remove ../ stuff
        clean_path = path.replace('../', '')
        # Convert path/to/file.md to path-to-file
        slug = clean_path.replace('.md', '').replace('/', '-')
        return f'href="#{slug}"'

    # Pattern for relative links to .md files
    # href="02_syntax/01_bindings.md" -> href="#02_syntax-01_bindings"
    # Actually, we should probably handle simple names too.
    content = re.sub(r'href="([^"]+\.md)"', link_repl, content)

    with open(html_file, 'w', encoding='utf-8') as f:
        f.write(content)

if __name__ == "__main__":
    if len(sys.argv) > 1:
        fix_links(sys.argv[1])

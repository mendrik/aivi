#!/usr/bin/env python3
"""
AIVI Syntax Highlighter
Adds syntax highlighting to AIVI code blocks in HTML.
"""

import re
import sys

# AIVI keywords based on language spec
KEYWORDS = [
    'domain', 'over', 'type', 'class', 'module', 'export', 'use',
    'effect', 'do', 'if', 'then', 'else', 'when', 'pure', 'let', 'in',
    'True', 'False', 'None', 'Some', 'Ok', 'Err', 'Empty', 'Unit'
]

TYPES = [
    'Int', 'Float', 'Text', 'Bool', 'List', 'Option', 'Result',
    'Effect', 'Source', 'Table', 'Row', 'Query', 'Element', 'Attribute',
    'Date', 'Instant', 'Duration', 'Span', 'Rgb', 'Hsl', 'Vec2', 'Vec3',
    'User', 'Post', 'Account', 'Decimal', 'Children', 'Delta', 'Patch',
    'Http', 'File', 'Json', 'JsonSchema', 'Image', 'ImageData'
]

OPERATORS = ['→', '⇒', '▷', '⇐', '←', '≠', '≥', '≤', '∧', '∨', '…', '?', '+', '-', '*', '/']

def highlight_aivi(html):
    """Add syntax highlighting to AIVI code blocks only."""
    
    def highlight_block(match):
        full_match = match.group(0)
        code = match.group(1)
        
        # Skip if not an aivi block (check the opening tag)
        if 'aivi' not in full_match.split('>')[0]:
            return full_match
        
        # Highlight line comments (// to end of line)
        code = re.sub(
            r'(//[^\n]*)',
            r'<span class="comment">\1</span>',
            code
        )
        
        # Highlight block comments (/* */)
        code = re.sub(
            r'(/\*.*?\*/)',
            r'<span class="comment">\1</span>',
            code,
            flags=re.DOTALL
        )
        
        # Highlight strings (double quotes and backticks)
        code = re.sub(
            r'("(?:[^"\\]|\\.)*"|`[^`]*`)',
            r'<span class="string">\1</span>',
            code
        )
        
        # Highlight characters (single quotes)
        code = re.sub(
            r"('[^']')",
            r'<span class="string">\1</span>',
            code
        )
        
        # Highlight decorators (@word)
        code = re.sub(
            r'(@\w+)',
            r'<span class="decorator">\1</span>',
            code
        )
        
        # Highlight types (capitalized words)
        for t in TYPES:
            code = re.sub(
                rf'\b({t})\b',
                r'<span class="type">\1</span>',
                code
            )
        
        # Highlight keywords
        for kw in KEYWORDS:
            code = re.sub(
                rf'\b({kw})\b',
                r'<span class="keyword">\1</span>',
                code
            )
        
        # Highlight numbers
        code = re.sub(
            r'\b(\d+(?:\.\d+)?)\b',
            r'<span class="number">\1</span>',
            code
        )
        
        return f'<code class="sourceCode aivi">{code}</code>'
    
    # Match only code blocks that have 'aivi' in their class
    html = re.sub(
        r'<code[^>]*class="[^"]*aivi[^"]*"[^>]*>(.*?)</code>',
        highlight_block,
        html,
        flags=re.DOTALL
    )
    
    return html

def main():
    if len(sys.argv) != 2:
        print("Usage: highlight.py <html-file>")
        sys.exit(1)
    
    filepath = sys.argv[1]
    
    with open(filepath, 'r', encoding='utf-8') as f:
        html = f.read()
    
    html = highlight_aivi(html)
    
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(html)
    
    print(f"✓ Syntax highlighting applied to {filepath}")

if __name__ == '__main__':
    main()

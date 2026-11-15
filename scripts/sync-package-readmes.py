#!/usr/bin/env python3
"""Share content while retaining NuGet's Markdown-only rendering conventions."""
import re
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
BASE = 'https://github.com/7mlabs-workspace/sdk-numerology/blob/main/'
RAW = 'https://raw.githubusercontent.com/7mlabs-workspace/sdk-numerology/main/'
text = (ROOT / 'README.md').read_text()

def absolute(match):
    target = match.group(1)
    if target.startswith(('#', 'mailto:')) or '://' in target or target in ('LICENSE', 'THIRD-PARTY-NOTICES.txt'):
        return match.group(0)
    return ']('+BASE+target+')'

text = re.sub(r'\]\(([^)]+)\)', absolute, text)
for attribute in ('src', 'srcset'):
    text = text.replace(attribute+'="docs/', attribute+'="'+RAW+'docs/')
text = text.replace('href="docs/', 'href="'+BASE+'docs/')
for name in ('bindings/node/README.md', 'bindings/python/README.md'):
    (ROOT / name).write_text(text)

def picture(match):
    image = re.search(r'<img src="([^"]+)"[^>]*alt="([^"]+)"', match.group(0))
    assert image
    return '!['+image.group(2)+']('+image.group(1)+')'

nuget = re.sub(r'<picture>.*?</picture>', picture, text, flags=re.S)
nuget = re.sub(r'<a href="([^"]+)">\s*(.*?)\s*</a>',
               lambda m: '['+m.group(2)+']('+(BASE+'LICENSE' if m.group(1)=='LICENSE' else m.group(1))+')', nuget, flags=re.S)
nuget = re.sub(r'<summary>.*?</summary>', '', nuget, flags=re.S)
nuget = re.sub(r'</?(?:details|p)\b[^>]*>', '', nuget)
nuget = re.sub(r'<a id="[^"]+"></a>', '', nuget)
for old, new in (('#quick-start','#cách-gọi-cơ-bản'),('#features','#chức-năng-hiện-có'),('#development','#dựng-và-kiểm-thử')):
    nuget = nuget.replace(']('+old+')', ']('+new+')')
nuget = re.sub(r'\n{3,}', '\n\n', nuget)
assert '<picture>' not in nuget and '<details>' not in nuget
(ROOT / 'bindings/dotnet/SevenMLabs.Numerology/README.md').write_text(nuget)
print('Synchronized package README content; preserved NuGet Markdown rendering.')

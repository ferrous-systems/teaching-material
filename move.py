from pathlib import Path
import shutil

def copy(src, dst):
    print(f'{src} -> {dst}')

    shutil.copy(src, dst)

base = Path('../rust-three-days-course/')
dest = Path('../teaching-material/')
pres = base / 'presentation/chapters/en-US/'

# for chapter in pres.glob('*.chapter'):
#     name = chapter.stem
#     destdir = dest / 'presentations' / name
#     if not destdir.exists():
#         destdir.mkdir()
#         slides = destdir / 'slides.adoc'
#         copy(chapter, slides)
#         code = base / 'presentation' / 'chapters' / 'shared' / 'code' / name
#         print(code)
#         for snippet in code.glob('*'):
#             copy(snippet, destdir / snippet.name)

old_header = "---\n\n## "
new_header = "== "
for chapter in (dest / 'presentations').iterdir():
    if chapter.is_file():
        continue
    slides = chapter / 'slides.adoc'
    old = open(slides).read()

    tt = "[Table of Contents](toc/english.html)"
    toc = old.find(tt)
    if toc == -1:
        continue

    header_len = toc + len(tt)
    old_header = old[:header_len].strip()
    name = old_header.splitlines()[0][1:].strip()
    new_header = f"""= {name}
:revealjs_width: 1920
:revealjs_height: 1080
:source-highlighter: highlightjs

link:./index.html[Table of Contents]
"""
    new = new_header  + old[header_len:]
    open(slides, 'w').write(new)

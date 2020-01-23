from pathlib import Path
import shutil
import re

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
    new = re.sub(
        r'<pre><code data-source="chapters/shared/code/.*/(.*)" data-trim="hljs rust" class="lang-rust"></code></pre>',
        "[source,rust]\n----\ninclude::./\\1[]\n----",
        old
    )
    open(slides, 'w').write(new)

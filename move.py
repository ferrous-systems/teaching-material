from pathlib import Path
import shutil

def copy(src, dst):
    print(f'{src} -> {dst}')

    shutil.copy(src, dst)

base = Path('../rust-three-days-course/')
dest = Path('../teaching-material/')
pres = base / 'presentation/chapters/en-US/'

for chapter in pres.glob('*.chapter'):
    name = chapter.stem
    destdir = dest / 'presentations' / name
    if not destdir.exists():
        destdir.mkdir()
        slides = destdir / 'slides.adoc'
        copy(chapter, slides)
        code = base / 'presentation' / 'chapters' / 'shared' / 'code' / name
        print(code)
        for snippet in code.glob('*'):
            copy(snippet, destdir / snippet.name)

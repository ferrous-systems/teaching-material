TARGET_DIR='target'
autoload :FileUtils, 'fileutils'
require 'asciidoctor'
require 'asciidoctor-diagram'
require 'asciidoctor-revealjs'

# TODO: add ability to build only a single assignment/presentation:
# https://stackoverflow.com/questions/9539324

def convert_assignments(doc, assignment_dir)
  puts "Converting #{doc}"
  Asciidoctor.convert_file doc,
  safe: :unsafe,
  attributes: <<-ATTRS,
    icons=font
    source-highlighter=rouge
    mermaid=node_modules/.bin/mmdc
  ATTRS
  to_file: "#{TARGET_DIR}/#{assignment_dir}/#{File.basename doc, '.*'}.html",
  mkdirs: true
end

desc 'Build assignments'
task :assignments => :index do
  # convert regular assignment sheets
  (FileList.new './assignments/*.adoc').each do |doc|
    convert_assignments(doc, 'assignments')
  end

  # convert "fill in the blanks" style assignment documentation
  (FileList.new './fill_in_the_blanks/*.adoc').each do |doc|
    convert_assignments(doc, 'fill_in_the_blanks')
  end

end

desc 'Build presentations'
task :presentations => :index do
  # FIXME: this will break if there's a name collision
  FileUtils.cp Dir.glob('./presentations/*/*.{svg,jpg}'), './target'
  FileUtils.cp './presentations/slides.css', './target/slides.css'

  (FileList.new './presentations/*/*.adoc').each do |doc|
    puts "Converting #{doc}"
    name = File.basename (File.dirname doc)
    Asciidoctor.convert_file doc,
      safe: :unsafe,
      attributes: <<-ATTRS,
        customcss=slides.css
        revealjsdir=https://cdn.jsdelivr.net/npm/reveal.js@4.3.1
        highlightjs-theme=https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.15.10/styles/idea.min.css
        icons=font
        revealjs_height=1080
        revealjs_history=true
        revealjs_theme=simple
        revealjs_transition=none
        revealjs_width=1920
        source-highlighter=highlightjs
        docinfo=shared
        docinfodir=../
        mermaid=node_modules/.bin/mmdc
      ATTRS
      backend: 'revealjs',
      to_file: "#{TARGET_DIR}/#{name}.html",
      mkdirs: true
  end
end

desc 'Build index'
task :index do
  Asciidoctor.convert_file './presentations/index.adoc',
    safe: :unsafe,
    to_file: "#{TARGET_DIR}/index.html",
    mkdirs: true
end

desc 'Build all'
task default: [:assignments, :presentations]

desc 'Clean the build directory'
task :clean do
  FileUtils.remove_entry_secure TARGET_DIR if File.exist? TARGET_DIR
end

# This file is responsible for requiring the compiled rust binary
# If it notices that there is not yet a compiled binary, it will attempt to compile it for you

unless Dir.glob("#{File.dirname(__FILE__)}/marc/marc.*").any?
  Rails.application.load_tasks
  Rake::Task['compile'].invoke
end

require_relative 'marc/marc'

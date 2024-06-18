# frozen_string_literal: true

require 'sorbet-runtime'

Dir[File.join(__dir__, 'katas', '**', '*.rb')].sort.each { |file| require_relative file }

# frozen_string_literal: true

Dir[File.join(__dir__, 'katas', '**', '*.rb')].sort.each { |file| require_relative file }

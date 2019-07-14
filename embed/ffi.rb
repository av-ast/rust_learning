require 'ffi'

module Hello
  extend FFI::Library
  ffi_lib "target/release/libembed.#{FFI::Platform::LIBSUFFIX}"
  attach_function :process, [], :void
end

Hello.process

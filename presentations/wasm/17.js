'use strict';

var library = {
  get_data: function() {
    var str = "Hello from JS.";
    alert(str);

    // Not needed for numerics.
    var len = lengthBytesUTF8(str);
    var buffer = Module._malloc(len);
    Module.stringToUTF8(str, buffer, len);

    return buffer;
  },
};

mergeInto(LibraryManager.library, library);
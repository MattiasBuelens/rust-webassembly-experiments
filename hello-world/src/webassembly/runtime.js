mergeInto(LibraryManager.library, {
  console_log: function(message_ptr, message_len) {
    var message = Pointer_stringify(message_ptr, message_len);
    console.log(message);
  },
  console_warn: function(message_ptr, message_len) {
    var message = Pointer_stringify(message_ptr, message_len);
    console.warn(message);
  },
  Math_sqrt: function(num) {
    return Math.sqrt(num);
  }
});
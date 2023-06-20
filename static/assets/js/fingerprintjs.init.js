  // Initialize the agent at application startup.
  var fpPromise = FingerprintJS.load()

  // Analyze the visitor when necessary.
  fpPromise
    .then(fp => fp.get())
    .then(result => {
        $('#fingerprint_token').val(result.visitorId)
        $('input[name=fingerprint_token]').each(function(){
            $(this).val(result.visitorId)
        })
    })
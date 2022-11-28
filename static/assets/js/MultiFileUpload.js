jQuery(function($){

	if( $("input[type=file].with-preview").length ){
	    $("input.file-upload-input").MultiFile({
	        list: ".file-upload-previews",
            afterFileRemove: function(element, value, master_element) {
                console.log(element)
                console.log(value)
                // $('#F9-Log').append('<li>afterFileRemove - ' + value + '</li>')
            }
	    })
	}

    $('.file-upload-previews').on('click','#remove_img', function(e) {
        // Keep  item click from being executed.
        e.stopPropagation()
        // Prevent navigating to '#'.
        e.preventDefault()
        // Ask user if he is sure.

        const $item = $(this).closest('.MultiFile-label')
        // $('#previous_image').val($item.find("img").attr('src'))
        // console.log($('#previous_image').val())

        $('.file-upload').show()
        $item.remove()
    })
})

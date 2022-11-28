$('#modalComponet').on('show.bs.modal', function (event){
    if(event.namespace == 'bs.modal'){
        var button = $(event.relatedTarget)
        var modal = $(this)
        var form = $('form',modal)[0]
        var inputs = modal.find('.modal-body input')
        var selects = modal.find('.modal-body select')
        var textareas = modal.find('.modal-body textarea')
            modal.find('.modal-title').text(button.data('action') +' '+$title)
            if(button.data('action') == 'Edit')
            {  
                // const input = $('<input name="_method" value="PUT" type="hidden"/>')
                // form.appendChild(input[0])
                const param = {inputs,textareas,selects,button}
                form.action = realUrl.replace('_', button.data('content').id)
                editModal(param)
            }
            else
            {
                $('.MultiFile-label').hide()
                inputs.val('')
                selects.val('').trigger('change')
            }
    }
})
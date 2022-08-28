$title = 'slot'
//$url = '/admin/slots/'
function editModal({selects,button}){
    const content = button.data('content')
    const position = content.position.id
    const nominees = content.nominees.map((nominee)=>nominee.id)
    $('option[value='+position+']',selects[0]).attr('selected',true)
    $('.select2').val(nominees).trigger('change')
}
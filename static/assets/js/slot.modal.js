$title = 'slot'
function editModal({selects, button}){
    const content = button.data('content')
    let [slot, nominees] = content
    nominees = nominees.map((nominee)=>nominee.id)
    $('option[value='+slot.position_id+']', selects[0]).attr('selected', true)
    $('.select2').val(nominees).trigger('change')
}
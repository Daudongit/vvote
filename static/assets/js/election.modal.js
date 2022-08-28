$title = 'election'
function editModal({inputs,button}){
    const content = button.data('content')
    const slots = content.slots.map((slot)=>slot.id)
    inputs[0].value = content.title
    inputs[2].value = content.start
    inputs[3].value = content.end
    $('.select2').val(slots).trigger('change')
}
$title = 'election'
function editModal({inputs,button}){
    const content = button.data('content')
    let [election, slots] = content
    inputs[0].value = election.title
    inputs[2].value = election.start
    inputs[3].value = election.end
    $('.select2').val(slots).trigger('change')
}
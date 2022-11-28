$title = 'Office'
function editModal({inputs,button}){
    const content = button.data('content')
    inputs[0].value = content.name
}

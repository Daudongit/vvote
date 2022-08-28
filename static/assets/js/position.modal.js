$title = 'Office'
function editModal({inputs,button}){
    content = button.data('content')
    inputs[0].value = content.name
}
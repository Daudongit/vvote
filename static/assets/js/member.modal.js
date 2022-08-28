$title = 'Member'
function editModal({inputs,button}){
    content = button.data('content')
    inputs[0].value = content.member_id
}
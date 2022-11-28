$title = 'nominee'
function editModal({inputs,button}){
    const content = button.data('content')
    let image_path = content.image || "/static/assets/image/no_image.jpg"
    if (!image_path.startsWith('http') && !image_path.includes('no_image')) {
        image_path = `/static/uploads/${content.image}`
    }

    inputs[0].value = content.first_name
    inputs[1].value = content.last_name
    inputs[2].value = content.email
    inputs[3].value = content.description
    inputs[4].value = content.image
    $('.MultiFile-preview')[0].src = image_path
    $('.MultiFile-title')[0].innerText = image_path
}
$title = 'nominee'
function editModal({inputs,button}){
    const content = button.data('content')
    const image_name = content.image
    const title = image_name.replace(new RegExp("\.jpg|\.png$"), '')
    let image_path = image_name || "/static/assets/image/no_image.jpg"
    if (!image_path.startsWith('http') && !image_path.includes('no_image')) {
        image_path = `/static/uploads/${image_name}`
    }

    inputs[0].value = content.first_name
    inputs[1].value = content.last_name
    inputs[2].value = content.email
    inputs[3].value = content.description
    inputs[4].value = image_name
    $('.MultiFile-preview')[0].src = image_path
    $('.MultiFile-title')[0].innerText = title
}

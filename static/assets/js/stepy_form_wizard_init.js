//step wizard
$(function() {
    let form = $('#default')
    form.stepy({
        backLabel: 'Previous',
        block: true,
        nextLabel: 'Next',
        titleClick: true,
        titleTarget: '.stepy-tab',
        next: function(index, totalSteps) {
            // scroll down
            // $("html, body").animate({
            //     scrollTop: $(document).height()
            // }, 9000);                  

            // /scroll back up
            $("html, body").animate({
                scrollTop: 0
            }, 500); 
        },
        validate:true,
        validateOptions:{
            errorPlacement: function(error, element) {},
            highlight: function(element, errorClass) {
                $('.alert').removeClass('hidden');
                $('.alert').fadeOut(1000,function() {
                    $('.alert').fadeIn();
                });
            },
            unhighlight: function(element, errorClass) {
                $('.alert').addClass('hidden');
            }
        }
    });
});
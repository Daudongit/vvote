function sweet_alert($callback,$title='Are you sure ?') {
	swal({
			title:$title,

			type:'warning',

			text:'You want to delete this.!',

			showCancelButton:true,

			confirmButtonColor: '#f44336',

			confirmButtonText:'Yes, delete!',

			closeOnConfirm:true

	},function(){$callback();});
}


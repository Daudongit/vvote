//! Admin module.

use jelly::actix_web::web::{get, post, put, patch, delete, resource, scope, ServiceConfig};
use jelly::guards::Auth;

pub mod views;
pub mod forms;

pub fn configure(config: &mut ServiceConfig) {
    let _guard = Auth {
        redirect_to: "/login/",
    };

    // config.service(
    //     scope("/dashboard/")
    //         .wrap(guard)
    //         // Index
    //         .service(resource("").to(views::dashboard)),
    // );
    config.service(
        scope("/admin/")
            .service(
                resource("/dashboard/")
                    .route(get().to(views::dashboard::index)).name("admin.dashboard"),
            )
            .service(
                resource("/results/")
                    .route(get().to(views::result::index)).name("admin.results.index"),
            )
            .service(
                resource("/results/{election}/")
                    .route(get().to(views::result::show)).name("admin.results.show"),
            )
            .service(
                resource("/suspend/{election}/")
                    .route(patch().to(views::suspend::update)).name("admin.suspend.update"),
            )
            .service(
                resource("/export/vote/{election}/{position}/")
                    .route(get().to(views::export::index)).name("admin.export.index"),
            ) // query param {type?}
            .service(
                resource("/export/{election}/{slot}/")
                    .route(get().to(views::export::show)).name("admin.export.show"),
            ) // query param {type?}
            .service(
                scope("/nominees/")
                    .service(
                        resource("")
                            .route(get().to(views::nominee::index))
                            .name("admin.nominees.index"),
                            // .route(post().to(views::nominee::store))
                    )
                    .service(
                        resource("/store/")
                        .route(post().to(views::nominee::store))
                        .name("admin.nominees.store"),
                    )
                    // .service(resource("/create/").route(get().to(views::nominee::create)),)
                    .service(
                        resource("/{id}/")
                            // .route(get().to(views::nominee::show))
                            .route(put().to(views::nominee::update))
                            .route(delete().to(views::nominee::destroy))
                            .name("admin.nominees.update.delete"),
                    )
                    // .service(resource("/{id}/edit/").route(get().to(views::nominee::edit)),)
            )
            .service(
                scope("/elections/")
                    .service(
                        resource("")
                            .route(get().to(views::election::index))
                            .route(post().to(views::election::store))
                            .name("admin.elections.index.store"),
                    )
                    .service(
                        resource("/{id}/")
                            .route(put().to(views::election::update))
                            .route(delete().to(views::election::destroy))
                            .name("admin.elections.update.destroy"),
                    )
            )
            .service(
                scope("/positions/")
                    .service(
                        resource("")
                            .route(get().to(views::position::index))
                            .route(post().to(views::position::store))
                            .name("admin.positions.index.store"),
                    )
                    .service(
                        resource("/{id}/")
                            .route(put().to(views::position::update))
                            .route(delete().to(views::position::destroy))
                            .name("admin.positions.update.destroy"),
                    )
            )
            .service(
                scope("/slots/")
                    .service(
                        resource("")
                            .route(get().to(views::slot::index))
                            .route(post().to(views::slot::store))
                            .name("admin.slots.index.store"),
                    )
                    .service(
                        resource("/{id}/")
                            .route(put().to(views::slot::update))
                            .route(delete().to(views::slot::destroy))
                            .name("admin.slots.update.destroy"),
                    )
            )
    );
}






// Route::group(['namespace'=>'Admin','prefix'=>'admin','middleware'=>'auth:web'],function(){
//     Route::get('dashboard','DashboardController@index')->name('admin.dashboard');
//     Route::get('results','ResultController@index')->name('admin.results.index');
//     Route::get('results/{election}','ResultController@show')->name('admin.results.show');
//     //Route::get('export/{election}/{slot}/{type?}','ResultController@exportElection')->name('admin.results.export');
//     Route::get('export/vote/{election}/{position}/{type?}','ExportController@index')->name('admin.export.votes');
//     Route::get('export/{election}/{slot}/{type?}','ExportController@show')->name('admin.export.election');

//     Route::resource('nominees','NomineeController',['as'=>'admin','except'=>['create','edit']]);
//     Route::resource('elections','ElectionController',['as'=>'admin','except'=>['create','edit','show']]);
//     Route::resource('positions','PositionController',['as'=>'admin','except'=>['create','edit']]);
//     Route::resource('slots','SlotController',['as'=>'admin','except'=>['create','edit']]);
//     Route::resource('members','MemberController',['as'=>'admin','except'=>['create','edit']]);
//     Route::get('codes','CodeController@index')->name('admin.codes.index');
//     Route::post('codes','CodeController@store')->name('admin.codes.store');
//     Route::patch('elections/toggle/{election}','ElectionController@toggle')->name('admin.elections.toggle');
//     Route::patch('elections/see/{election}','ElectionController@see')->name('admin.elections.see');
// });


// Route.get('users', 'UserController.index').as('users.index')
// Route.post('users', 'UserController.store').as('users.store')
// Route.get('users/create', 'UserController.create').as('users.create')
// Route.get('users/:id', 'UserController.show').as('users.show')
// Route.put('users/:id', 'UserController.update').as('users.update')
// Route.patch('users/:id', 'UserController.update')
// Route.get('users/:id/edit', 'UserController.edit').as('users.edit')
// Route.delete('users/:id', 'UserController.destroy').as('users.destroy'

// articles.index
// articles.create
// articles.store
// articles.show
// articles.edit
// articles.update
// articles.destroy

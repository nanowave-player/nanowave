use crate::App;
use slint::ComponentHandle;

pub fn setup(app: &App) {

    let weak = app.as_weak();
    // let cfg = config.clone();
    /*
    app.global::<HomeState>().on_check_internet(move || {
        let Some(app) = weak.upgrade() else { return };
        let net = archinstall_zfs_core::system::net::check_internet();
        app.global::<WelcomeState>().set_net_ok(net);
        if net {
            if !app.global::<WelcomeState>().get_zfs_ok()
                && !app.global::<WelcomeState>().get_zfs_installing()
            {
                start_zfs_init(&app, &cfg.borrow());
            }
            if !kscan.is_some() {
                start_kernel_scan(&kscan);
            }
        }
    });

     */
}

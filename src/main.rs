use serde::Deserialize;

use crate::router::routers::register_handlers;

mod node_exporter {
    pub mod mem_utils {
        pub mod meminfo;
    }
    pub mod cpu_utils {
        pub mod cpuinfo;
        pub mod cpuloadavg;
    }
    pub mod disk_utils {
        pub mod diskinfo;
    }
    pub mod file_utils {
        pub mod fileinfo;
    }
    pub mod proc_utils {
        pub mod process;
    }
}

mod mysql_exporter{
    pub mod check_and_link;
    pub mod query_indicators;
    pub mod innodb;
    pub mod binlog;
    pub mod character;
}

mod hand {
    pub mod node {
        pub mod file_operation;
        pub mod firewall;
        pub mod user;
    }
}

mod api {
    pub mod node_exporter {
        pub mod linux_process_api;
        pub mod linux_cpu_api;
        pub mod linux_memory_api;
        pub mod linux_disk_api;
        pub mod linux_network_api;
    }
    pub mod linux_file_action_api;
}

mod router {
    pub mod routers;
}

#[tokio::main]
async fn main() {
    // 注册路由
    let app = register_handlers();

    // run our app with hyper, listening globally on port 4201
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4201").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
use std::thread::sleep;

use libapuadj::*;
fn main() {
    println!("Hello, world!");

    let adj = unsafe { init_ryzenadj() };
    if adj.is_null() {
        panic!("adj is null")
    };

    let table = unsafe { init_table(adj) };
    println!("table: {:?}", table);
    //if table==0 { panic!("table is null") };
    loop {
        unsafe { refresh_table(adj) };

        let cpu_family = unsafe { get_cpu_family(adj) };
        println!("cpu_family: {}", cpu_family);
        let tctl_temp = unsafe { get_tctl_temp(adj) };
        println!("tctl_temp: {}", tctl_temp);
        let apu_skin_temp_limit = unsafe { get_apu_skin_temp_limit(adj) };
        println!("apu_skin_temp_limit: {}", apu_skin_temp_limit);
        //持续功耗限制
        let stapm_limit = unsafe { get_stapm_limit(adj) };
        let stamp_value = unsafe { get_stapm_value(adj) };
        println!("stapm-limit: {},stamp-value: {}", stapm_limit, stamp_value);
        //瞬时功耗限制
        let fast_limit = unsafe { get_fast_limit(adj) };
        let fast_value = unsafe { get_fast_value(adj) };
        println!("fast-limit: {},fast-value: {}", fast_limit, fast_value);
        //峰值功耗限制
        let slow_limit = unsafe { get_slow_limit(adj) };
        let slow_value = unsafe { get_slow_value(adj) };
        println!("slow-limit: {},slow-value: {}", slow_limit, slow_value);

        //设置功耗限制
        let power_limit = 10.0 * 1000.0;
        
        if (stapm_limit * 1000.0) as u32 != power_limit as u32  {
            println!("🚀stapm_limit is {}, set power limit to: {}", stapm_limit * 1000.0,power_limit);
            unsafe { set_stapm_limit(adj, power_limit as u32) };
            unsafe { set_fast_limit(adj, (power_limit * 1.2) as u32) };
            unsafe { set_slow_limit(adj, (power_limit * 1.5) as u32) };
        }

        sleep(std::time::Duration::from_secs(15));
    }
}

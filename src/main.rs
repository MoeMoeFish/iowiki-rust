/*
 * @Author: MoeMoeFish moemoefish@qq.com
 * @Date: 2022-10-24 18:18:36
 * @LastEditors: MoeMoeFish moemoefish@qq.com
 * @LastEditTime: 2022-10-25 13:27:43
 * @Description: 
 */
mod runner;
mod dp;

use dp::digit_dp::digit_dp_01;
use dp::basic_dp::basic_dp_01;


fn main() {
    digit_dp_01::run();
    basic_dp_01::run();
    println!("Hello, world!");

}

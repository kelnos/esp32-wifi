/* automatically generated by rust-bindgen */

#![allow(non_camel_case_types)]
#![allow(broken_intra_doc_links)]

pub type __uint8_t = cty::c_uchar;
pub type __uint32_t = cty::c_uint;
#[doc = " @brief Structure holding PHY init parameters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esp_phy_init_data_t {
    #[doc = "< opaque PHY initialization parameters"]
    pub params: [u8; 128usize],
}
#[doc = " @brief Opaque PHY calibration data"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esp_phy_calibration_data_t {
    #[doc = "< PHY version"]
    pub version: [u8; 4usize],
    #[doc = "< The MAC address of the station"]
    pub mac: [u8; 6usize],
    #[doc = "< calibration data"]
    pub opaque: [u8; 1894usize],
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum esp_phy_calibration_mode_t {
    #[doc = "< Do part of RF calibration. This should be used after power-on reset."]
    PHY_RF_CAL_PARTIAL = 0,
    #[doc = "< Don't do any RF calibration. This mode is only suggested to be used after deep sleep reset."]
    PHY_RF_CAL_NONE = 1,
    #[doc = "< Do full RF calibration. Produces best results, but also consumes a lot of time and current. Suggested to be used once."]
    PHY_RF_CAL_FULL = 2,
}
extern "C" {
    #[doc = " @brief Return ROM function pointer table from PHY library."]
    pub fn phy_get_romfunc_addr();
}
extern "C" {
    #[doc = " @brief Initialize PHY module and do RF calibration"]
    #[doc = " @param[in] init_data Initialization parameters to be used by the PHY"]
    #[doc = " @param[inout] cal_data As input, calibration data previously obtained. As output, will contain new calibration data."]
    #[doc = " @param[in] cal_mode  RF calibration mode"]
    #[doc = " @return ESP_CAL_DATA_CHECK_FAIL if calibration data checksum fails, other values are reserved for future use"]
    pub fn register_chipv7_phy(
        init_data: *const esp_phy_init_data_t,
        cal_data: *mut esp_phy_calibration_data_t,
        cal_mode: esp_phy_calibration_mode_t,
    ) -> cty::c_int;
}
extern "C" {
    #[doc = " @brief Get the format version of calibration data used by PHY library."]
    #[doc = " @return Format version number, OR'ed with BIT(16) if PHY is in WIFI only mode."]
    pub fn phy_get_rf_cal_version() -> u32;
}
extern "C" {
    #[doc = " @brief Set RF/BB for only WIFI mode or coexist(WIFI & BT) mode"]
    #[doc = " @param[in] true is for only WIFI mode, false is for coexist mode. default is 0."]
    #[doc = " @return NULL"]
    pub fn phy_set_wifi_mode_only(wifi_only: bool);
}
extern "C" {
    #[doc = " @brief Set BT the highest priority in coexist mode."]
    #[doc = " @return NULL"]
    pub fn coex_bt_high_prio();
}
extern "C" {
    #[doc = " @brief Shutdown PHY and RF."]
    pub fn phy_close_rf();
}

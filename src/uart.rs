#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x4025_0000],
    #[doc = "0x40250000 - Generic control register"]
    pub cy_ctrl: crate::Reg<cy_ctrl::CY_CTRL_SPEC>,
    _reserved1: [u8; 0x1c],
    #[doc = "0x40250020 - SPI control register"]
    pub cy_spi_ctrl: crate::Reg<cy_spi_ctrl::CY_SPI_CTRL_SPEC>,
    #[doc = "0x40250024 - SPI status register"]
    pub cy_spi_status: crate::Reg<cy_spi_status::CY_SPI_STATUS_SPEC>,
    _reserved3: [u8; 0x18],
    #[doc = "0x40250040 - UART control register"]
    pub cy_uart_ctrl: crate::Reg<cy_uart_ctrl::CY_UART_CTRL_SPEC>,
    #[doc = "0x40250044 - UART transmitter control register"]
    pub cy_uart_tx_ctrl: crate::Reg<cy_uart_tx_ctrl::CY_UART_TX_CTRL_SPEC>,
    #[doc = "0x40250048 - UART receiver control register"]
    pub cy_uart_rx_ctrl: crate::Reg<cy_uart_rx_ctrl::CY_UART_RX_CTRL_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x40250050 - UART flow control register"]
    pub cy_uart_flow_ctrl: crate::Reg<cy_uart_flow_ctrl::CY_UART_FLOW_CTRL_SPEC>,
    _reserved7: [u8; 0x0c],
    #[doc = "0x40250060 - I2C control register"]
    pub cy_i2c_ctrl: crate::Reg<cy_i2c_ctrl::CY_I2C_CTRL_SPEC>,
    #[doc = "0x40250064 - I2C status register"]
    pub cy_i2c_status: crate::Reg<cy_i2c_status::CY_I2C_STATUS_SPEC>,
    #[doc = "0x40250068 - I2C master command register"]
    pub cy_i2c_m_cmd: crate::Reg<cy_i2c_m_cmd::CY_I2C_M_CMD_SPEC>,
    #[doc = "0x4025006c - I2C slave command register"]
    pub cy_i2c_s_cmd: crate::Reg<cy_i2c_s_cmd::CY_I2C_S_CMD_SPEC>,
    #[doc = "0x40250070 - I2C fitler trimm register"]
    pub cy_i2c_cfg: crate::Reg<cy_i2c_cfg::CY_I2C_CFG_SPEC>,
    _reserved12: [u8; 0x018c],
    #[doc = "0x40250200 - Transmitter control register"]
    pub cy_tx_ctrl: crate::Reg<cy_tx_ctrl::CY_TX_CTRL_SPEC>,
    #[doc = "0x40250204 - Transmitter FIFO control register"]
    pub cy_tx_fifo_ctrl: crate::Reg<cy_tx_fifo_ctrl::CY_TX_FIFO_CTRL_SPEC>,
    #[doc = "0x40250208 - Transmitter FIFO status register"]
    pub cy_tx_fifo_status: crate::Reg<cy_tx_fifo_status::CY_TX_FIFO_STATUS_SPEC>,
    _reserved15: [u8; 0x34],
    #[doc = "0x40250240 - Transmitter FIFO write register"]
    pub cy_tx_fifo_wr: crate::Reg<cy_tx_fifo_wr::CY_TX_FIFO_WR_SPEC>,
    _reserved16: [u8; 0xbc],
    #[doc = "0x40250300 - Receiver control register"]
    pub cy_rx_ctrl: crate::Reg<cy_rx_ctrl::CY_RX_CTRL_SPEC>,
    #[doc = "0x40250304 - Receiver FIFO control register"]
    pub cy_rx_fifo_ctrl: crate::Reg<cy_rx_fifo_ctrl::CY_RX_FIFO_CTRL_SPEC>,
    #[doc = "0x40250308 - Receiver FIFO status registerS"]
    pub cy_rx_fifo_status: crate::Reg<cy_rx_fifo_status::CY_RX_FIFO_STATUS_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0x40250310 - Slave address and mask register"]
    pub cy_rx_match: crate::Reg<cy_rx_match::CY_RX_MATCH_SPEC>,
    _reserved20: [u8; 0x2c],
    #[doc = "0x40250340 - Receiver FIFO read register"]
    pub cy_rx_fifo_rd: crate::Reg<cy_rx_fifo_rd::CY_RX_FIFO_RD_SPEC>,
    _reserved21: [u8; 0x0abc],
    #[doc = "0x40250e00 - Interrupt cause register"]
    pub cy_intr_cause: crate::Reg<cy_intr_cause::CY_INTR_CAUSE_SPEC>,
    _reserved22: [u8; 0x7c],
    #[doc = "0x40250e80 - Externally clocked I2C interrupt request register"]
    pub cy_intr_i2c_ec: crate::Reg<cy_intr_i2c_ec::CY_INTR_I2C_EC_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x40250e88 - Externally clocked I2C interrupt mask register"]
    pub cy_intr_i2c_ec_mask: crate::Reg<cy_intr_i2c_ec_mask::CY_INTR_I2C_EC_MASK_SPEC>,
    #[doc = "0x40250e8c - Externally clocked SPI interrupt masked register"]
    pub cy_intr_i2c_ec_masked: crate::Reg<cy_intr_i2c_ec_masked::CY_INTR_I2C_EC_MASKED_SPEC>,
    _reserved25: [u8; 0x30],
    #[doc = "0x40250ec0 - Externally clocked SPI interrupt request register"]
    pub cy_intr_intr_spi_ec: crate::Reg<cy_intr_intr_spi_ec::CY_INTR_INTR_SPI_EC_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0x40250ec8 - Externally clocked SPI interrupt mask register"]
    pub cy_intr_intr_spi_ec_mask:
        crate::Reg<cy_intr_intr_spi_ec_mask::CY_INTR_INTR_SPI_EC_MASK_SPEC>,
    #[doc = "0x40250ecc - Externally clocked SPI interrupt masked register"]
    pub cy_intr_intr_spi_ec_masked:
        crate::Reg<cy_intr_intr_spi_ec_masked::CY_INTR_INTR_SPI_EC_MASKED_SPEC>,
    _reserved28: [u8; 0x30],
    #[doc = "0x40250f00 - Master interrupt request register."]
    pub cy_intr_m: crate::Reg<cy_intr_m::CY_INTR_M_SPEC>,
    #[doc = "0x40250f04 - Master interrupt set request register"]
    pub cy_intr_m_set: crate::Reg<cy_intr_m_set::CY_INTR_M_SET_SPEC>,
    #[doc = "0x40250f08 - Master interrupt mask register"]
    pub cy_intr_m_mask: crate::Reg<cy_intr_m_mask::CY_INTR_M_MASK_SPEC>,
    #[doc = "0x40250f0c - Master interrupt masked request register"]
    pub cy_intr_m_masked: crate::Reg<cy_intr_m_masked::CY_INTR_M_MASKED_SPEC>,
    _reserved32: [u8; 0x30],
    #[doc = "0x40250f40 - Slave interrupt request register"]
    pub cy_intr_s: crate::Reg<cy_intr_s::CY_INTR_S_SPEC>,
    #[doc = "0x40250f44 - Slave interrupt set request register"]
    pub cy_intr_s_set: crate::Reg<cy_intr_s_set::CY_INTR_S_SET_SPEC>,
    #[doc = "0x40250f48 - Slave interrupt mask register"]
    pub cy_intr_s_mask: crate::Reg<cy_intr_s_mask::CY_INTR_S_MASK_SPEC>,
    #[doc = "0x40250f4c - Slave interrupt masked register"]
    pub cy_intr_s_masked: crate::Reg<cy_intr_s_masked::CY_INTR_S_MASKED_SPEC>,
    _reserved36: [u8; 0x30],
    #[doc = "0x40250f80 - Transmitter interrupt request register"]
    pub cy_intr_tx: crate::Reg<cy_intr_tx::CY_INTR_TX_SPEC>,
    #[doc = "0x40250f84 - Transmitter interrupt set request register"]
    pub cy_intr_tx_set: crate::Reg<cy_intr_tx_set::CY_INTR_TX_SET_SPEC>,
    #[doc = "0x40250f88 - Transmitter interrupt mask request register"]
    pub cy_intr_tx_mask: crate::Reg<cy_intr_tx_mask::CY_INTR_TX_MASK_SPEC>,
    #[doc = "0x40250f8c - Transmitter interrupt masked request register"]
    pub cy_intr_tx_masked: crate::Reg<cy_intr_tx_masked::CY_INTR_TX_MASKED_SPEC>,
    _reserved40: [u8; 0x30],
    #[doc = "0x40250fc0 - Receiver interrupt request register"]
    pub cy_intr_rx: crate::Reg<cy_intr_rx::CY_INTR_RX_SPEC>,
    #[doc = "0x40250fc4 - Receiver interrupt set request register"]
    pub cy_intr_rx_set: crate::Reg<cy_intr_rx_set::CY_INTR_RX_SET_SPEC>,
    #[doc = "0x40250fc8 - Receiver interrupt mask register"]
    pub cy_intr_rx_mask: crate::Reg<cy_intr_rx_mask::CY_INTR_RX_MASK_SPEC>,
    #[doc = "0x40250fcc - Receiver interrupt masked register"]
    pub cy_intr_rx_masked: crate::Reg<cy_intr_rx_masked::CY_INTR_RX_MASKED_SPEC>,
}
#[doc = "Cy_CTRL register accessor: an alias for `Reg<CY_CTRL_SPEC>`"]
pub type CY_CTRL = crate::Reg<cy_ctrl::CY_CTRL_SPEC>;
#[doc = "Generic control register"]
pub mod cy_ctrl;
#[doc = "Cy_SPI_CTRL register accessor: an alias for `Reg<CY_SPI_CTRL_SPEC>`"]
pub type CY_SPI_CTRL = crate::Reg<cy_spi_ctrl::CY_SPI_CTRL_SPEC>;
#[doc = "SPI control register"]
pub mod cy_spi_ctrl;
#[doc = "Cy_SPI_STATUS register accessor: an alias for `Reg<CY_SPI_STATUS_SPEC>`"]
pub type CY_SPI_STATUS = crate::Reg<cy_spi_status::CY_SPI_STATUS_SPEC>;
#[doc = "SPI status register"]
pub mod cy_spi_status;
#[doc = "Cy_UART_CTRL register accessor: an alias for `Reg<CY_UART_CTRL_SPEC>`"]
pub type CY_UART_CTRL = crate::Reg<cy_uart_ctrl::CY_UART_CTRL_SPEC>;
#[doc = "UART control register"]
pub mod cy_uart_ctrl;
#[doc = "Cy_UART_TX_CTRL register accessor: an alias for `Reg<CY_UART_TX_CTRL_SPEC>`"]
pub type CY_UART_TX_CTRL = crate::Reg<cy_uart_tx_ctrl::CY_UART_TX_CTRL_SPEC>;
#[doc = "UART transmitter control register"]
pub mod cy_uart_tx_ctrl;
#[doc = "Cy_UART_RX_CTRL register accessor: an alias for `Reg<CY_UART_RX_CTRL_SPEC>`"]
pub type CY_UART_RX_CTRL = crate::Reg<cy_uart_rx_ctrl::CY_UART_RX_CTRL_SPEC>;
#[doc = "UART receiver control register"]
pub mod cy_uart_rx_ctrl;
#[doc = "Cy_UART_FLOW_CTRL register accessor: an alias for `Reg<CY_UART_FLOW_CTRL_SPEC>`"]
pub type CY_UART_FLOW_CTRL = crate::Reg<cy_uart_flow_ctrl::CY_UART_FLOW_CTRL_SPEC>;
#[doc = "UART flow control register"]
pub mod cy_uart_flow_ctrl;
#[doc = "Cy_I2C_CTRL register accessor: an alias for `Reg<CY_I2C_CTRL_SPEC>`"]
pub type CY_I2C_CTRL = crate::Reg<cy_i2c_ctrl::CY_I2C_CTRL_SPEC>;
#[doc = "I2C control register"]
pub mod cy_i2c_ctrl;
#[doc = "Cy_I2C_STATUS register accessor: an alias for `Reg<CY_I2C_STATUS_SPEC>`"]
pub type CY_I2C_STATUS = crate::Reg<cy_i2c_status::CY_I2C_STATUS_SPEC>;
#[doc = "I2C status register"]
pub mod cy_i2c_status;
#[doc = "Cy_I2C_M_CMD register accessor: an alias for `Reg<CY_I2C_M_CMD_SPEC>`"]
pub type CY_I2C_M_CMD = crate::Reg<cy_i2c_m_cmd::CY_I2C_M_CMD_SPEC>;
#[doc = "I2C master command register"]
pub mod cy_i2c_m_cmd;
#[doc = "Cy_I2C_S_CMD register accessor: an alias for `Reg<CY_I2C_S_CMD_SPEC>`"]
pub type CY_I2C_S_CMD = crate::Reg<cy_i2c_s_cmd::CY_I2C_S_CMD_SPEC>;
#[doc = "I2C slave command register"]
pub mod cy_i2c_s_cmd;
#[doc = "Cy_I2C_CFG register accessor: an alias for `Reg<CY_I2C_CFG_SPEC>`"]
pub type CY_I2C_CFG = crate::Reg<cy_i2c_cfg::CY_I2C_CFG_SPEC>;
#[doc = "I2C fitler trimm register"]
pub mod cy_i2c_cfg;
#[doc = "Cy_TX_CTRL register accessor: an alias for `Reg<CY_TX_CTRL_SPEC>`"]
pub type CY_TX_CTRL = crate::Reg<cy_tx_ctrl::CY_TX_CTRL_SPEC>;
#[doc = "Transmitter control register"]
pub mod cy_tx_ctrl;
#[doc = "Cy_TX_FIFO_CTRL register accessor: an alias for `Reg<CY_TX_FIFO_CTRL_SPEC>`"]
pub type CY_TX_FIFO_CTRL = crate::Reg<cy_tx_fifo_ctrl::CY_TX_FIFO_CTRL_SPEC>;
#[doc = "Transmitter FIFO control register"]
pub mod cy_tx_fifo_ctrl;
#[doc = "Cy_TX_FIFO_STATUS register accessor: an alias for `Reg<CY_TX_FIFO_STATUS_SPEC>`"]
pub type CY_TX_FIFO_STATUS = crate::Reg<cy_tx_fifo_status::CY_TX_FIFO_STATUS_SPEC>;
#[doc = "Transmitter FIFO status register"]
pub mod cy_tx_fifo_status;
#[doc = "Cy_TX_FIFO_WR register accessor: an alias for `Reg<CY_TX_FIFO_WR_SPEC>`"]
pub type CY_TX_FIFO_WR = crate::Reg<cy_tx_fifo_wr::CY_TX_FIFO_WR_SPEC>;
#[doc = "Transmitter FIFO write register"]
pub mod cy_tx_fifo_wr;
#[doc = "Cy_RX_CTRL register accessor: an alias for `Reg<CY_RX_CTRL_SPEC>`"]
pub type CY_RX_CTRL = crate::Reg<cy_rx_ctrl::CY_RX_CTRL_SPEC>;
#[doc = "Receiver control register"]
pub mod cy_rx_ctrl;
#[doc = "Cy_RX_FIFO_CTRL register accessor: an alias for `Reg<CY_RX_FIFO_CTRL_SPEC>`"]
pub type CY_RX_FIFO_CTRL = crate::Reg<cy_rx_fifo_ctrl::CY_RX_FIFO_CTRL_SPEC>;
#[doc = "Receiver FIFO control register"]
pub mod cy_rx_fifo_ctrl;
#[doc = "Cy_RX_FIFO_STATUS register accessor: an alias for `Reg<CY_RX_FIFO_STATUS_SPEC>`"]
pub type CY_RX_FIFO_STATUS = crate::Reg<cy_rx_fifo_status::CY_RX_FIFO_STATUS_SPEC>;
#[doc = "Receiver FIFO status registerS"]
pub mod cy_rx_fifo_status;
#[doc = "Cy_RX_MATCH register accessor: an alias for `Reg<CY_RX_MATCH_SPEC>`"]
pub type CY_RX_MATCH = crate::Reg<cy_rx_match::CY_RX_MATCH_SPEC>;
#[doc = "Slave address and mask register"]
pub mod cy_rx_match;
#[doc = "Cy_RX_FIFO_RD register accessor: an alias for `Reg<CY_RX_FIFO_RD_SPEC>`"]
pub type CY_RX_FIFO_RD = crate::Reg<cy_rx_fifo_rd::CY_RX_FIFO_RD_SPEC>;
#[doc = "Receiver FIFO read register"]
pub mod cy_rx_fifo_rd;
#[doc = "Cy_INTR_CAUSE register accessor: an alias for `Reg<CY_INTR_CAUSE_SPEC>`"]
pub type CY_INTR_CAUSE = crate::Reg<cy_intr_cause::CY_INTR_CAUSE_SPEC>;
#[doc = "Interrupt cause register"]
pub mod cy_intr_cause;
#[doc = "Cy_INTR_I2C_EC register accessor: an alias for `Reg<CY_INTR_I2C_EC_SPEC>`"]
pub type CY_INTR_I2C_EC = crate::Reg<cy_intr_i2c_ec::CY_INTR_I2C_EC_SPEC>;
#[doc = "Externally clocked I2C interrupt request register"]
pub mod cy_intr_i2c_ec;
#[doc = "Cy_INTR_I2C_EC_MASK register accessor: an alias for `Reg<CY_INTR_I2C_EC_MASK_SPEC>`"]
pub type CY_INTR_I2C_EC_MASK = crate::Reg<cy_intr_i2c_ec_mask::CY_INTR_I2C_EC_MASK_SPEC>;
#[doc = "Externally clocked I2C interrupt mask register"]
pub mod cy_intr_i2c_ec_mask;
#[doc = "Cy_INTR_I2C_EC_MASKED register accessor: an alias for `Reg<CY_INTR_I2C_EC_MASKED_SPEC>`"]
pub type CY_INTR_I2C_EC_MASKED = crate::Reg<cy_intr_i2c_ec_masked::CY_INTR_I2C_EC_MASKED_SPEC>;
#[doc = "Externally clocked SPI interrupt masked register"]
pub mod cy_intr_i2c_ec_masked;
#[doc = "Cy_INTR_INTR_SPI_EC register accessor: an alias for `Reg<CY_INTR_INTR_SPI_EC_SPEC>`"]
pub type CY_INTR_INTR_SPI_EC = crate::Reg<cy_intr_intr_spi_ec::CY_INTR_INTR_SPI_EC_SPEC>;
#[doc = "Externally clocked SPI interrupt request register"]
pub mod cy_intr_intr_spi_ec;
#[doc = "Cy_INTR_INTR_SPI_EC_MASK register accessor: an alias for `Reg<CY_INTR_INTR_SPI_EC_MASK_SPEC>`"]
pub type CY_INTR_INTR_SPI_EC_MASK =
    crate::Reg<cy_intr_intr_spi_ec_mask::CY_INTR_INTR_SPI_EC_MASK_SPEC>;
#[doc = "Externally clocked SPI interrupt mask register"]
pub mod cy_intr_intr_spi_ec_mask;
#[doc = "Cy_INTR_INTR_SPI_EC_MASKED register accessor: an alias for `Reg<CY_INTR_INTR_SPI_EC_MASKED_SPEC>`"]
pub type CY_INTR_INTR_SPI_EC_MASKED =
    crate::Reg<cy_intr_intr_spi_ec_masked::CY_INTR_INTR_SPI_EC_MASKED_SPEC>;
#[doc = "Externally clocked SPI interrupt masked register"]
pub mod cy_intr_intr_spi_ec_masked;
#[doc = "Cy_INTR_M register accessor: an alias for `Reg<CY_INTR_M_SPEC>`"]
pub type CY_INTR_M = crate::Reg<cy_intr_m::CY_INTR_M_SPEC>;
#[doc = "Master interrupt request register."]
pub mod cy_intr_m;
#[doc = "Cy_INTR_M_SET register accessor: an alias for `Reg<CY_INTR_M_SET_SPEC>`"]
pub type CY_INTR_M_SET = crate::Reg<cy_intr_m_set::CY_INTR_M_SET_SPEC>;
#[doc = "Master interrupt set request register"]
pub mod cy_intr_m_set;
#[doc = "Cy_INTR_M_MASK register accessor: an alias for `Reg<CY_INTR_M_MASK_SPEC>`"]
pub type CY_INTR_M_MASK = crate::Reg<cy_intr_m_mask::CY_INTR_M_MASK_SPEC>;
#[doc = "Master interrupt mask register"]
pub mod cy_intr_m_mask;
#[doc = "Cy_INTR_M_MASKED register accessor: an alias for `Reg<CY_INTR_M_MASKED_SPEC>`"]
pub type CY_INTR_M_MASKED = crate::Reg<cy_intr_m_masked::CY_INTR_M_MASKED_SPEC>;
#[doc = "Master interrupt masked request register"]
pub mod cy_intr_m_masked;
#[doc = "Cy_INTR_S register accessor: an alias for `Reg<CY_INTR_S_SPEC>`"]
pub type CY_INTR_S = crate::Reg<cy_intr_s::CY_INTR_S_SPEC>;
#[doc = "Slave interrupt request register"]
pub mod cy_intr_s;
#[doc = "Cy_INTR_S_SET register accessor: an alias for `Reg<CY_INTR_S_SET_SPEC>`"]
pub type CY_INTR_S_SET = crate::Reg<cy_intr_s_set::CY_INTR_S_SET_SPEC>;
#[doc = "Slave interrupt set request register"]
pub mod cy_intr_s_set;
#[doc = "Cy_INTR_S_MASK register accessor: an alias for `Reg<CY_INTR_S_MASK_SPEC>`"]
pub type CY_INTR_S_MASK = crate::Reg<cy_intr_s_mask::CY_INTR_S_MASK_SPEC>;
#[doc = "Slave interrupt mask register"]
pub mod cy_intr_s_mask;
#[doc = "Cy_INTR_S_MASKED register accessor: an alias for `Reg<CY_INTR_S_MASKED_SPEC>`"]
pub type CY_INTR_S_MASKED = crate::Reg<cy_intr_s_masked::CY_INTR_S_MASKED_SPEC>;
#[doc = "Slave interrupt masked register"]
pub mod cy_intr_s_masked;
#[doc = "Cy_INTR_TX register accessor: an alias for `Reg<CY_INTR_TX_SPEC>`"]
pub type CY_INTR_TX = crate::Reg<cy_intr_tx::CY_INTR_TX_SPEC>;
#[doc = "Transmitter interrupt request register"]
pub mod cy_intr_tx;
#[doc = "Cy_INTR_TX_SET register accessor: an alias for `Reg<CY_INTR_TX_SET_SPEC>`"]
pub type CY_INTR_TX_SET = crate::Reg<cy_intr_tx_set::CY_INTR_TX_SET_SPEC>;
#[doc = "Transmitter interrupt set request register"]
pub mod cy_intr_tx_set;
#[doc = "Cy_INTR_TX_MASK register accessor: an alias for `Reg<CY_INTR_TX_MASK_SPEC>`"]
pub type CY_INTR_TX_MASK = crate::Reg<cy_intr_tx_mask::CY_INTR_TX_MASK_SPEC>;
#[doc = "Transmitter interrupt mask request register"]
pub mod cy_intr_tx_mask;
#[doc = "Cy_INTR_TX_MASKED register accessor: an alias for `Reg<CY_INTR_TX_MASKED_SPEC>`"]
pub type CY_INTR_TX_MASKED = crate::Reg<cy_intr_tx_masked::CY_INTR_TX_MASKED_SPEC>;
#[doc = "Transmitter interrupt masked request register"]
pub mod cy_intr_tx_masked;
#[doc = "Cy_INTR_RX register accessor: an alias for `Reg<CY_INTR_RX_SPEC>`"]
pub type CY_INTR_RX = crate::Reg<cy_intr_rx::CY_INTR_RX_SPEC>;
#[doc = "Receiver interrupt request register"]
pub mod cy_intr_rx;
#[doc = "Cy_INTR_RX_SET register accessor: an alias for `Reg<CY_INTR_RX_SET_SPEC>`"]
pub type CY_INTR_RX_SET = crate::Reg<cy_intr_rx_set::CY_INTR_RX_SET_SPEC>;
#[doc = "Receiver interrupt set request register"]
pub mod cy_intr_rx_set;
#[doc = "Cy_INTR_RX_MASK register accessor: an alias for `Reg<CY_INTR_RX_MASK_SPEC>`"]
pub type CY_INTR_RX_MASK = crate::Reg<cy_intr_rx_mask::CY_INTR_RX_MASK_SPEC>;
#[doc = "Receiver interrupt mask register"]
pub mod cy_intr_rx_mask;
#[doc = "Cy_INTR_RX_MASKED register accessor: an alias for `Reg<CY_INTR_RX_MASKED_SPEC>`"]
pub type CY_INTR_RX_MASKED = crate::Reg<cy_intr_rx_masked::CY_INTR_RX_MASKED_SPEC>;
#[doc = "Receiver interrupt masked register"]
pub mod cy_intr_rx_masked;

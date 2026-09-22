use bitflags::bitflags;

bitflags! {
     #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
     pub struct CORECFGR: usize {
          /// 浮点加法分频
          ///
          /// # 可用架构
          /// * V3F
          /// * V5
          const FADD_CLKDIV_MASK = 0xF << 28;

          /// 浮点乘法分频
          ///
          /// # 可用架构
          /// * V3F
          /// * V5
          const FMUL_CLKDIV_MASK = 0xF << 24;

          /// 浮点乘加分频
          ///
          /// # 可用架构
          /// * V3F
          /// * V5
          const FMAC_CLKDIV_MASK = 0xF << 20;

          /// 浮点除法分频
          ///
          /// # 可用架构
          /// * V3F
          /// * V5
          const FDIV_CLKDIV_MASK = 0xF << 16;

          /// 下一行分支预测功能使能
          ///
          /// # 可用架构
          /// * V5
          const NLP_EN = 1 << 15;

          /// 全局历史寄存器功能使能
          ///
          /// # 可用架构
          /// * V5
          const GHR_EN = 1 << 14;

          /// 访存指令并行发射
          ///
          /// # 可用架构
          /// * V5
          const LSU_DUAL = 1 << 10;

          /// 内核状态错误中断使能
          const CSTA_FAULT_IE = 1 << 7;

          /// fence屏蔽中断使能
          const INT_FENCE = 1 << 6;

          /// MIE寄存器映射使能
          const IE_REMAP_EN = 1 << 5;

          /// 循环加速
          ///
          /// # 可用架构
          /// * V3(BCV)
          const ROM_LOOP_ACC = 1 << 3;

          /// 跳转加速
          ///
          /// # 可用架构
          /// * V3(BCV)
          const ROM_JUMP_ACC = 1 << 2;

          /// 指令预取模式1
          const FETCH_MODE_1 = 0b01 << 0;
          /// 指令预取关闭
          const FETCH_MODE_0 = 0b00 << 0;
          /// 指令预取模式
          ///
          /// # 可用架构
          /// * V3
          const FETCH_MODE_MASK = 0x3 << 0;
     }
}

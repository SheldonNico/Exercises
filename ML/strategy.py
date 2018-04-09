from omega.backTest import Signal
from omega.dataBase import EOD, generate_date, generate_logicalDate
import numpy as np

class MACD(Signal):
    signal = 0
    max_cached = 50

    def __init__(self, param_long=26, param_short=12, param_macd=9):
        self.param_long = param_long
        self.param_short = param_short
        self.param_macd = param_macd

        self.max_cached += param_long

    def prebatch(self, info):
        if not info.isSingle:
            raise ValueError("only work on single ticker")

        from omega.talib_nd import macd as ta_macd
        self.dif, self.dea, self.macd = ta_macd(
            self.eod.close,
            fastperiod = self.param_short,
            slowperiod = self.param_long,
            signalperiod = self.param_macd)

    def generate(self, di):
        if self.signal == 0:
            if self.dif[di-1] > 0 and self.dea[di-1] > 0 and self.dif[di-1] > self.dif[di-2] and self.dif[di-1] > self.dea[di-1]:
                self.signal = 1
            elif self.dif[di-1] < 0 and self.dea[di-1] < 0 and self.dif[di-1] < self.dif[di-2] and self.dif[di-1] < self.dea[di-1]:
                self.signal = -1

            if self.signal != 0:
                self.entryPrice = self.eod.open[di]
        elif self.signal > 0:
            if self.eod.low[di-1] < self.entryPrice * 0.995:
                self.signal = 0
            if self.dif[di-1] < 0 and self.dea[di-1] < 0 and self.dif[di-1] < self.dif[di-2] and self.dif[di-1] < self.dea[di-1]:
                self.signal = 0
        else:
            if self.eod.high[di-1] > self.entryPrice * 1.005:
                self.signal = 0
            if self.dif[di-1] > 0 and self.dea[di-1] > 0 and self.dif[di-1] > self.dif[di-2] and self.dif[di-1] > self.dea[di-1]:
                self.signal = 0

        return self.signal

class ATR(Signal):
    signal = 0
    max_cached = 50

    def __init__(self, param_period=14):
        self.param_period = param_period

    def prebatch(self, info):
        if not info.isSingle:
            raise ValueError("only work on single ticker")

        from omega.talib_nd import atr as ta_atr
        from omega.talib_nd import ma as ta_ma
        atr = ta_atr(self.eod.high, self.eod.low, self.eod.close, self.param_period)
        ma = ta_ma(self.eod.close, self.param_period)
        self.high_limit = ma + 2 * atr
        self.low_limit = ma - 2 * atr
        self.ma = ma

    def generate(self, di):
        if self.signal == 0:
            if self.eod.high[di-1] > self.high_limit[di-1]:
                self.signal = -1
            elif self.eod.low[di-1] < self.low_limit[di-1]:
                self.signal = 1

            if self.signal != 0:
                self.entryPrice = self.eod.open[di]
        elif self.signal > 0:
            if self.eod.low[di-1] < self.entryPrice * 0.995:
                self.signal = 0

            if self.eod.close[di-1] > self.ma[di-1]:
                self.signal = 0
        else:
            if self.eod.high[di-1] > self.entryPrice * 1.005:
                self.signal = 0

            if self.eod.close[di-1] < self.ma[di-1]:
                self.signal = 0

        return -self.signal


class DualThrust(Signal):
    signal = 0
    max_cached = 10

    def __init__(self, param_K1=0.5, param_K2=0.5):
        self.param_K1 = param_K1
        self.param_K2 = param_K2

    def prebatch(self, info):
        if not info.isSingle:
            raise ValueError("This strategy works only for single kind")

        # eod = EOD(info.ticker, "1day")
        eod = self.eod.reFreq("1day")

        self.dayHL = np.max([eod.high-eod.close, eod.high-eod.open, eod.open-eod.low, eod.close-eod.low], axis=0) * 0.5
        self.dayHLIndex = eod.time
        self.getloc = lambda dt: eod.loc(dt, direction="right") - 1

        self.date = generate_date(self.eod.time)
        self.logicalDate = generate_logicalDate(self.eod.time)
        self.lastDate = eod.time[1].astype("O")
        self.highLimit = None

    def generate(self, di):
        if ((self.logicalDate[di] != self.logicalDate[di-1]) or (self.date[di] != self.date[di-1])):
            # print("Update for date", self.time[di])
            dayOpen = self.eod.open[di]
            dayDi = self.getloc(self.eod.time[di]) # today
            # print(self.eod.time[di], self.dayHLIndex[dayDi-1])
            self.highLimit = dayOpen + self.dayHL[dayDi-1] * self.param_K2
            self.lowLimit = dayOpen - self.dayHL[dayDi-1] * self.param_K1
            return self.signal

        if self.highLimit is None:
            return self.signal


        if self.signal == 0:
            if self.eod.high[di-1] > self.highLimit:
                self.signal = 1
            elif self.eod.low[di-1] < self.lowLimit:
                self.signal = -1
        elif self.signal > 0:
            if self.eod.low[di-1] < self.lowLimit:
                self.signal = 0
        else:
            if self.eod.high[di-1] > self.highLimit:
                self.signal = 0
        return self.signal

if __name__ == "__main__":
    import matplotlib.pyplot as plt
    sig = DualThrust()
    report = sig.run("ru888", "15min", "20100101", "20180101")
    report.plot()
    plt.show()

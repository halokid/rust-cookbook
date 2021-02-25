package main

import (
  "log"
  "testing"
)

func TestRemoveFromSlice(t *testing.T) {
  is := []int{1, 2, 3, 4, 5, 6, 7}
  isx := removeFromSlice(is, 3)   // todo: 去掉下标为3的元素， 即是去掉4
  t.Logf("isx ------------- %+v", isx)
}

func removeFromSlice(is []int, i int) []int {
  iLen := len(is)
  for _, isItem := range is {
    log.Printf("isItem ------------- %+v", isItem)
    if isItem == i {
      // todo: 快速remove slice元素的算法， 把最后一位元素补位给当前要被remove的元素， 然后返回整个新的slice
      is[iLen - 1], is[i] = is[i], is[iLen - 1]
      log.Printf("(iLen - 1)------------- %+v", iLen - 1)
      return is[:iLen - 1]    // todo: 不取最后一位，则成功去掉4， 因为上一步已经把4移到了最后一位
      //return is[:iLen]    // todo:   如果是这样那最后一位就是4
    }
  }
  return is
}

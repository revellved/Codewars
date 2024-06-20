package com.revellved.codewars.katas;

import org.junit.Test;
import static org.junit.Assert.assertEquals;
import org.junit.runners.JUnit4;

// TODO: Replace examples and use TDD development by writing your own tests

public class ReverseWordsTest {
  @Test
  public void exampleCases() {
    assertEquals("ehT kciuq nworb xof spmuj revo eht yzal .god",
        ReverseWords.exec("The quick brown fox jumps over the lazy dog."));
    assertEquals("elppa", ReverseWords.exec("apple"));
    assertEquals("a b c d", ReverseWords.exec("a b c d"));
    assertEquals("elbuod  decaps  sdrow", ReverseWords.exec("double  spaced  words"));
    assertEquals("", ReverseWords.exec(""));
    assertEquals("     ", ReverseWords.exec("     "));
  }
}

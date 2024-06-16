package com.revellved.codewars;

/**
 * reverse Words
 *
 */

public class ReverseWords
{
  public static String exec(final String original)
  {
    String reverseStr = "";
    for (String part : original.split(" ")) {
        reverseStr += new StringBuilder(part).reverse().toString() + " ";
    }
    if (reverseStr.length() == 0) {
      return original;
    }
    return reverseStr.substring(0, reverseStr.length()-1);
  }
}

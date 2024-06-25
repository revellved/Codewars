import { narcissistic } from './solution';
import * as chai from 'chai';
import 'mocha'

const assert = chai.assert;
chai.config.truncateThreshold=0;

describe( "Example tests", function() {
  
  function dotest(input: number, expected: boolean) {
    const actual = narcissistic(input);
    assert.strictEqual(actual, expected, `Incorrect answer for value=${input}`)
  }
  
  it("Narcissistic numbers", function() {
    dotest(  7, true);
    dotest(153, true);
  });
  
  it("Not narcissistic numbers", function() {
    dotest(122, false);
    dotest(487, false);
  });
});

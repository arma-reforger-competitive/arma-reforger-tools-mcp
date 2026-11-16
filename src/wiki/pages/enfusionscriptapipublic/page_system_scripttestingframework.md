---
title: Script Testing Framework
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_System_ScriptTestingFramework.html
---

## Introduction

- Provides a unified and simple interface that emphasizes the smallest amount of boiler plate code possible.
- Each test is marked using Test attribute. A test is a type derived from TestBase. It can run for several ticks, maintaining state using member variables. Your logic has to be ran through **step methods**.
- Each test specifies to which TestSuite it belongs. Suites provide additional API for controlling the execution environment (for all tests which are part of a suite).
- Within the framework a SINGLE class derived from TestHarness can exist. TestHarness is collection of all available test suites. It runs tests in them and provides API to access them.

### Step methods

- You can name your step methods however you like.
- They have to be annotated with TestStep attribute that specifies TestStage to which this step belongs.

### Stages

- They divide the steps into groups that express the initialization and finalization process.
- Under normal circumstances (no failure or error occurs), stages are executed in following order:
  1. TestStage.Setup
  1. TestStage.Main
  1. TestStage.TearDown
- Methods in stages are executed in order of definition.

### Return values

- void -> Will get executed only once.
- bool -> Will get executed every tick until true is returned.

### Result

- Result is set via TestBase.SetResult(TestResultBase).
- Setting a result which evaluates to failure will terminate the stage.

### Failure unwind

- If the TestStage.Setup fails the test only terminates and TestStage.TearDown is not called.
- Failure during TestStage.Main will trigger the TestStage.TearDown.
- Failure during TestStage.TearDown will do nothing.

### Timeout

- Tests won't timeout by default. The value may be specified via Test or TestStep attributes.
- Test attribute specifies default timeout for steps of a test.
- Non-zero timeout specified in TestStep overrides timeout from Test.
- TestSuite steps may also have their
- The timeout timer resets for every step method.
- If the step method times out the TimeoutResult is set (it evaluates to failure and the failure unwind process starts).

```
[Test(suite: MyTestSuite, timeoutS: 2, timeoutMs: 250)]
class MyAsyncTest : TestBase
{
    private int m_Count;
 
    // Simple blocking initialization.
    [TestStep(TestStage.Setup)]
    void Init()
    {
        Print("MyAsyncTest.Init");
        m_Count = 10;
    }
 
    // Async test which is waiting for result for several frames.
    [TestStep(TestStage.Main)]
    bool Poll()
    {
        Print("MyAsyncTest.Poll: " + m_Count);
 
        if (m_Count == 0)
        {
            Print("MyAsyncTest.Poll Result");
            SetFailure(new TestFailureMsg("Intentionally failed."));
            m_Count = 3;
            return true;
        }
 
        Print("MyAsyncTest.Poll Progress");
 
        m_Count--;
        return false;
    }
 
    // Finalization process waiting for result for several frames.
    [TestStep(TestStage.TearDown)]
    bool Finalizing()
    {
        Print("MyAsyncTest.Finalizing: " + m_Count);
        if (m_Count == 0)
        {
            Print("MyAsyncTest.Finalizing Done");
            return true;
        }
 
        Print("MyAsyncTest.Finalizing Progress");
        m_Count--;
        return false;
    }
 
    // Simple blocking finalization call.
    [TestStep(TestStage.TearDown)]
    void Finalized()
    {
        Print("MyAsyncTest.Finalized");
    }
}
```

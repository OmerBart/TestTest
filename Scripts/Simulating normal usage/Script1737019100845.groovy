import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
// Wait for a specific element to be visible
import java.io.File as File
import java.io.FileWriter as FileWriter

WebUI.openBrowser('')

WebUI.navigateToUrl('https://qa.caretuner.com/CT-admin/clin-login-1.php')

WebUI.openBrowser('')

WebUI.navigateToUrl('https://qa.caretuner.com/CT-admin/clin-login-1.php')

WebUI.setText(findTestObject('Object Repository/simulating/Page_Login/input_Username_username'), '253z74Au')

WebUI.setEncryptedText(findTestObject('Object Repository/simulating/Page_Login/input_Password_password'), 'swlpJnst+Cvdb1/MZTnD0g==')

WebUI.click(findTestObject('Object Repository/simulating/Page_Login/button_Log in'))

WebUI.click(findTestObject('Object Repository/simulating/Page_Care Tuner/span_Bart Simpson'))

if (WebUI.waitForElementVisible(findTestObject('Object Repository/simulating/Page_Care Tuner/a_View Diary'), 10)) {
    WebUI.click(findTestObject('Object Repository/simulating/Page_Care Tuner/a_View Diary'))

    long startTime = System.currentTimeMillis()
	WebUI.delay(5)
    WebUI.comment('Element was found and clicked.')
} else {
    WebUI.comment('Element not found within the timeout.')
}



WebUI.click(findTestObject('Object Repository/simulating/Page_Care Tuner/a_View Diary'))

WebUI.delay(5)

WebUI.click(findTestObject('Object Repository/simulating/Page_Care Tuner/a_CareTuner.com_fancybox-item fancybox-close'))

WebUI.click(findTestObject('Object Repository/simulating/Page_Care Tuner/a_Logout'))

writer.write(('Step execution time: ' + executionTime) + ' ms\n')

writer.close()

File file = new File('ExecutionTimes.txt')

FileWriter writer = new FileWriter(file, true)

WebUI.comment('Execution time saved to file.')

// Add a delay of 5 seconds
if (WebUI.waitForElementVisible(findTestObject('Object Repository/Test/Page_Care Tuner/div_Assessments'), 10)) {
    WebUI.click(findTestObject('Object Repository/Test/Page_Care Tuner/div_Assessments'))

    WebUI.comment('Element was found and clicked.')
} else {
    WebUI.comment('Element not found within the timeout.')
}

// Add a delay of 5 seconds
WebUI.delay(5)


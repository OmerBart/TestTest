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
import java.io.File
import java.io.FileWriter
import java.time.LocalDateTime
import java.time.format.DateTimeFormatter

WebUI.openBrowser('')

WebUI.navigateToUrl('https://qa.caretuner.com/CT-admin/clin-login-1.php')

WebUI.setText(findTestObject('Object Repository/Page_Login/input_Username_username'), '253z74Au')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Login/input_Password_password'), 'swlpJnst+Cvdb1/MZTnD0g==')

WebUI.click(findTestObject('Object Repository/Page_Login/button_Log in'))

WebUI.click(findTestObject('Object Repository/Page_Care Tuner/span_Bart Simpson'))
LocalDateTime now = LocalDateTime.now()
DateTimeFormatter formatter = DateTimeFormatter.ofPattern("HH:mm:ss")
String currentDateTime = now.format(formatter)
String currentDir = System.getProperty("user.dir")
String rtmAction = ""
int lineLength = 80
String separator = "-".repeat(lineLength)


if (WebUI.waitForElementVisible(findTestObject('Object Repository/Page_Care Tuner/a_View Diary'), 10)) {
	rtmAction = "View Diary"
	WebUI.click(findTestObject('Object Repository/Page_Care Tuner/a_View Diary'))
	long startTime = System.currentTimeMillis()
	WebUI.comment('Element was found and clicked.')
	WebUI.delay(2)
	long endTime = System.currentTimeMillis()
	
	long executionTime = endTime - startTime
	File file = new File("ExecutionTimes.txt")
	FileWriter writer = new FileWriter(file, true) // Append mode
	writer.write(separator + "\n") // Full line of dashes above
	writer.write("Current Directory: " + currentDir + "\n") // Add current directory
	writer.write("Date: " + currentDateTime + " || " + rtmAction + " Execution time: " + executionTime + " ms\n")
	writer.write(separator + "\n\n") // Full line of dashes below with spacing
	writer.close()
	
	WebUI.comment("Execution time saved to file.")
} else {
	WebUI.comment('Element not found within the timeout.')
}

WebUI.click(findTestObject('Object Repository/Page_Care Tuner/a_CareTuner.com_fancybox-item fancybox-close'))

WebUI.click(findTestObject('Object Repository/Page_Care Tuner/a_Logout'))

WebUI.closeBrowser()


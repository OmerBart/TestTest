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

WebUI.openBrowser('')

WebUI.navigateToUrl('https://qa.caretuner.com/CT-admin/clin-login-1.php')

WebUI.click(findTestObject('Object Repository/Page_Login/input_Username_username'))

WebUI.setText(findTestObject('Object Repository/Page_Login/input_Username_username'), '253z74Au')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Login/input_Password_password'), 'swlpJnst+Cvdb1/MZTnD0g==')

WebUI.click(findTestObject('Object Repository/Page_Login/button_Log in'))

WebUI.delay(3)

WebUI.click(findTestObject('Object Repository/Page_Login/body_Two-Factor Authentication Your account is protected with two-factor authentication. Input the code we sent to 972XXXXXXX74 to access your account.Resend codeVerifyOr, Enter a one-time code     jQuery(function() jQue'))

WebUI.click(findTestObject('Object Repository/Page_Login/button_Verify'))

WebUI.delay(5)

WebUI.closeBrowser()


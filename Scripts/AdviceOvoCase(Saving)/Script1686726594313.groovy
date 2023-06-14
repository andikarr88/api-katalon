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

signatureResponse = WS.sendRequest(findTestObject('Postman/Advice/GET SIGNATURE SERVICE - ovo Advice Payment'))

signature = WS.getElementPropertyValue(signatureResponse, 'signature')

println('signature service is :' + signature)

GlobalVariable.SERVICE_SIGNATURE = signature

danaAdviceResponse = WS.sendRequest(findTestObject('Postman/Advice/ovo Payment Advice'))

RC = WS.getElementPropertyValue(danaAdviceResponse, 'responseCode')

println('RC is : ' + RC)

WS.verifyResponseStatusCode(danaAdviceResponse, 200)

String responseBody = danaAdviceResponse.getResponseBodyContent()
// Menampilkan response body di console
println("Response Body:")
println(responseBody)
from selenium import webdriver
from selenium.webdriver.common.keys import Keys

driver = webdriver.Firefox()
driver.get("https://www.andersenstories.com/da/andersen_fortaellinger/list")
links = driver.find_elements_by_xpath("//div[@class='main']//ul[@class='bluelink']/li/span/a")
hrefs = []
for link in links:
    href = link.get_property("href")
    hrefs.append(href)
for href in hrefs:
    driver.get(href)
    eventyr = driver.find_element_by_xpath("//div[@class='content']//div[@class='main']//div[@id='plainText']")
    titel = eventyr.find_element_by_xpath("//h1[@class='title']").text
    tekst = eventyr.find_element_by_xpath("//div[@class='text']").text
    file = open("eventyr/"+titel+".txt", "w")
    file.write(tekst.replace("<br>", "\n"))
    file.close()
driver.close()

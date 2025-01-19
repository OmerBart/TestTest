<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Two-Factor Authentication Your account is protected with two-factor authentication. Input the code we sent to 972XXXXXXX74 to access your account.Resend codeVerifyOr, Enter a one-time code     jQuery(function() jQue</name>
   <tag></tag>
   <elementGuidId>99faf4c3-de37-48e7-8e56-7a88f0a64191</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>body</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>12d12bb5-b9c2-4556-b35a-d725895a6b72</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
		
		
		
			Two-Factor Authentication
		
		
		 
			Your account is protected with two-factor authentication. Input the code we sent to +972XXXXXXX74 to access your account.
			
			
			
			
			
			Resend code
			
			Verify
			
			Or, Enter a one-time code
				
		  
		   
		
		
		
			
				jQuery(function() {
					jQuery('#xoken').val('').focus();
				});
				function keep_this_iframe(data)
				{	
					add_str = '';
										
					if (data == 'OK') alert('Code Resent');
					else alert('Cannot resend code. Please refresh this page.' + add_str);
				}
				
				function resend_token(e)
				{
					jQuery('input[name=&quot;action&quot;]').val(&quot;s&quot;);
					form = jQuery(e).parents('form:first');

					submit_form(form, 'keep_this_iframe');
					return(false);					
				}
				
				function check_token(e)
				{
					jQuery('input[name=&quot;action&quot;]').val('w');
					form = jQuery(e).parents('form:first');
				
					submit_form(form, '');
					return(false);
				}
				function submit_form(form, action_function)
				{
					var url = form.attr('action');
										jQuery.ajax({
						   type: &quot;POST&quot;,
						   cache		: false,
						   url: url,
						   data: form.serialize(), 
						   success: function(data)
						   {
																act_f = &quot;&quot;;
								if (act_f.length > 4) {	
									ret = eval(act_f)(data);
								}	
								else if (action_function.length > 4) {
									//ret = eval(action_function)(data);
									fn = window[action_function];  //avoid eval
									if (typeof fn === &quot;function&quot;) ret = fn(data);
									
								}
								else {

									//	if (&quot;qa.caretuner.com&quot; == 'localhost' &amp;&amp; 1 == 1) data = 'OK';
										if (1 == 1) data = 'OK';
									   if (data == 'NOK') alert('Cannot verify. Please enter code again.');
									   if (data == &quot;OK&quot;)  {
											//alert('Code Verified.');
											 parent.all_good_to_go = true;
										   	 parent.jQuery.fancybox.close(true);

									   }
									  }
							}
						 });
					
					return(false);
				}				
					
		
		
				
			function code_verified(data)
			{
				//alert('IN CODE VERIFIED: '+data);
				
			}
		
		</value>
      <webElementGuid>afd7f943-b7fd-4094-b6c5-5ab9c72c9056</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>1f549058-4403-4138-8d53-e4cf9363368e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>ref_element</name>
      <type>Main</type>
      <value>Object Repository/Page_Login/iframe_concat(id(, , fancybox-frame17367462_965a30</value>
      <webElementGuid>6c279c27-c68c-40da-aae6-8920224ec397</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>dc3e9168-259d-4c3e-9186-5bec433b6c64</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
		
		
		
			Two-Factor Authentication
		
		
		 
			Your account is protected with two-factor authentication. Input the code we sent to +972XXXXXXX74 to access your account.
			
			
			
			
			
			Resend code
			
			Verify
			
			Or, Enter a one-time code
				
		  
		   
		
		
		
			
				jQuery(function() {
					jQuery(&quot; , &quot;'&quot; , &quot;#xoken&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).focus();
				});
				function keep_this_iframe(data)
				{	
					add_str = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
										
					if (data == &quot; , &quot;'&quot; , &quot;OK&quot; , &quot;'&quot; , &quot;) alert(&quot; , &quot;'&quot; , &quot;Code Resent&quot; , &quot;'&quot; , &quot;);
					else alert(&quot; , &quot;'&quot; , &quot;Cannot resend code. Please refresh this page.&quot; , &quot;'&quot; , &quot; + add_str);
				}
				
				function resend_token(e)
				{
					jQuery(&quot; , &quot;'&quot; , &quot;input[name=&quot;action&quot;]&quot; , &quot;'&quot; , &quot;).val(&quot;s&quot;);
					form = jQuery(e).parents(&quot; , &quot;'&quot; , &quot;form:first&quot; , &quot;'&quot; , &quot;);

					submit_form(form, &quot; , &quot;'&quot; , &quot;keep_this_iframe&quot; , &quot;'&quot; , &quot;);
					return(false);					
				}
				
				function check_token(e)
				{
					jQuery(&quot; , &quot;'&quot; , &quot;input[name=&quot;action&quot;]&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;w&quot; , &quot;'&quot; , &quot;);
					form = jQuery(e).parents(&quot; , &quot;'&quot; , &quot;form:first&quot; , &quot;'&quot; , &quot;);
				
					submit_form(form, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
					return(false);
				}
				function submit_form(form, action_function)
				{
					var url = form.attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;);
										jQuery.ajax({
						   type: &quot;POST&quot;,
						   cache		: false,
						   url: url,
						   data: form.serialize(), 
						   success: function(data)
						   {
																act_f = &quot;&quot;;
								if (act_f.length > 4) {	
									ret = eval(act_f)(data);
								}	
								else if (action_function.length > 4) {
									//ret = eval(action_function)(data);
									fn = window[action_function];  //avoid eval
									if (typeof fn === &quot;function&quot;) ret = fn(data);
									
								}
								else {

									//	if (&quot;qa.caretuner.com&quot; == &quot; , &quot;'&quot; , &quot;localhost&quot; , &quot;'&quot; , &quot; &amp;&amp; 1 == 1) data = &quot; , &quot;'&quot; , &quot;OK&quot; , &quot;'&quot; , &quot;;
										if (1 == 1) data = &quot; , &quot;'&quot; , &quot;OK&quot; , &quot;'&quot; , &quot;;
									   if (data == &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot;) alert(&quot; , &quot;'&quot; , &quot;Cannot verify. Please enter code again.&quot; , &quot;'&quot; , &quot;);
									   if (data == &quot;OK&quot;)  {
											//alert(&quot; , &quot;'&quot; , &quot;Code Verified.&quot; , &quot;'&quot; , &quot;);
											 parent.all_good_to_go = true;
										   	 parent.jQuery.fancybox.close(true);

									   }
									  }
							}
						 });
					
					return(false);
				}				
					
		
		
				
			function code_verified(data)
			{
				//alert(&quot; , &quot;'&quot; , &quot;IN CODE VERIFIED: &quot; , &quot;'&quot; , &quot;+data);
				
			}
		
		&quot;) or . = concat(&quot;
		
		
		
			Two-Factor Authentication
		
		
		 
			Your account is protected with two-factor authentication. Input the code we sent to +972XXXXXXX74 to access your account.
			
			
			
			
			
			Resend code
			
			Verify
			
			Or, Enter a one-time code
				
		  
		   
		
		
		
			
				jQuery(function() {
					jQuery(&quot; , &quot;'&quot; , &quot;#xoken&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).focus();
				});
				function keep_this_iframe(data)
				{	
					add_str = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
										
					if (data == &quot; , &quot;'&quot; , &quot;OK&quot; , &quot;'&quot; , &quot;) alert(&quot; , &quot;'&quot; , &quot;Code Resent&quot; , &quot;'&quot; , &quot;);
					else alert(&quot; , &quot;'&quot; , &quot;Cannot resend code. Please refresh this page.&quot; , &quot;'&quot; , &quot; + add_str);
				}
				
				function resend_token(e)
				{
					jQuery(&quot; , &quot;'&quot; , &quot;input[name=&quot;action&quot;]&quot; , &quot;'&quot; , &quot;).val(&quot;s&quot;);
					form = jQuery(e).parents(&quot; , &quot;'&quot; , &quot;form:first&quot; , &quot;'&quot; , &quot;);

					submit_form(form, &quot; , &quot;'&quot; , &quot;keep_this_iframe&quot; , &quot;'&quot; , &quot;);
					return(false);					
				}
				
				function check_token(e)
				{
					jQuery(&quot; , &quot;'&quot; , &quot;input[name=&quot;action&quot;]&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;w&quot; , &quot;'&quot; , &quot;);
					form = jQuery(e).parents(&quot; , &quot;'&quot; , &quot;form:first&quot; , &quot;'&quot; , &quot;);
				
					submit_form(form, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
					return(false);
				}
				function submit_form(form, action_function)
				{
					var url = form.attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;);
										jQuery.ajax({
						   type: &quot;POST&quot;,
						   cache		: false,
						   url: url,
						   data: form.serialize(), 
						   success: function(data)
						   {
																act_f = &quot;&quot;;
								if (act_f.length > 4) {	
									ret = eval(act_f)(data);
								}	
								else if (action_function.length > 4) {
									//ret = eval(action_function)(data);
									fn = window[action_function];  //avoid eval
									if (typeof fn === &quot;function&quot;) ret = fn(data);
									
								}
								else {

									//	if (&quot;qa.caretuner.com&quot; == &quot; , &quot;'&quot; , &quot;localhost&quot; , &quot;'&quot; , &quot; &amp;&amp; 1 == 1) data = &quot; , &quot;'&quot; , &quot;OK&quot; , &quot;'&quot; , &quot;;
										if (1 == 1) data = &quot; , &quot;'&quot; , &quot;OK&quot; , &quot;'&quot; , &quot;;
									   if (data == &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot;) alert(&quot; , &quot;'&quot; , &quot;Cannot verify. Please enter code again.&quot; , &quot;'&quot; , &quot;);
									   if (data == &quot;OK&quot;)  {
											//alert(&quot; , &quot;'&quot; , &quot;Code Verified.&quot; , &quot;'&quot; , &quot;);
											 parent.all_good_to_go = true;
										   	 parent.jQuery.fancybox.close(true);

									   }
									  }
							}
						 });
					
					return(false);
				}				
					
		
		
				
			function code_verified(data)
			{
				//alert(&quot; , &quot;'&quot; , &quot;IN CODE VERIFIED: &quot; , &quot;'&quot; , &quot;+data);
				
			}
		
		&quot;))]</value>
      <webElementGuid>71218f52-8c34-4b35-958d-cc6cb1e64024</webElementGuid>
   </webElementXpaths>
</WebElementEntity>

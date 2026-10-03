@ECHO OFF

%windir%/SYSNATIVE/dism /Online /enable-feature /FeatureName:"LegacyComponents" /NoRestart
%windir%/SYSNATIVE/dism /Online /enable-feature /FeatureName:"DirectPlay" /NoRestart

exit

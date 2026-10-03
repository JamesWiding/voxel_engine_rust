@ECHO OFF
SDBInst -q "JediKnightDarkForces2.sdb"

reg query "HKEY_LOCAL_MACHINE\SOFTWARE\Wow6432Node\LucasArts Entertainment Company\JediKnight\v1.0" /v "b3DAccel"
if ERRORLEVEL 1 REG ADD "HKEY_LOCAL_MACHINE\SOFTWARE\Wow6432Node\LucasArts Entertainment Company\JediKnight\v1.0" /v "b3DAccel" /t REG_BINARY /d 01000000 /f

reg query "HKEY_LOCAL_MACHINE\SOFTWARE\Wow6432Node\LucasArts Entertainment Company\JediKnight\v1.0" /v "displayMode"
if ERRORLEVEL 1 REG ADD "HKEY_LOCAL_MACHINE\SOFTWARE\Wow6432Node\LucasArts Entertainment Company\JediKnight\v1.0" /v "displayMode" /t REG_BINARY /d 0c000000 /f

reg query "HKEY_LOCAL_MACHINE\SOFTWARE\Wow6432Node\LucasArts Entertainment Company\JediKnight\v1.0" /v "b3DAccel"
if ERRORLEVEL 1 REG ADD "HKEY_LOCAL_MACHINE\SOFTWARE\Wow6432Node\LucasArts Entertainment Company\JediKnight\v1.0" /v "b3DAccel" /t REG_BINARY /d 01000000 /f

reg query "HKEY_LOCAL_MACHINE\SOFTWARE\Wow6432Node\LucasArts Entertainment Company\JediKnight\v1.0" /v "displayMode"
if ERRORLEVEL 1 REG ADD "HKEY_LOCAL_MACHINE\SOFTWARE\Wow6432Node\LucasArts Entertainment Company\JediKnight\v1.0" /v "displayMode" /t REG_BINARY /d 0c000000 /f

SLEEP 5

exit

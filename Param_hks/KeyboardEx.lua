
-- // ===========================================================================
-- Keyboard Extension
-- Autor: VIZZO
-- Version: 0.2
-- Private Release: 28/09/2025
--
-- LOG (v0.02)
-- Adicionado suporte para gamepad.
-- Adicionado novos efeitos para suportar entradas de gamepad.
-- DLL modificada para suportar entradas de gamepad.
--
-- DESCRIÇÃO:
-- Detecção de teclas do teclado para uso em mods do Elden Ring.
-- Permite detectar teclas individuais, combos simultâneos e combos sequenciais
-- através de um sistema de cache inteligente.
--
-- CARACTERÍSTICAS:
-- - Suporte para teclas individuais (A, W, 1, F1, etc.)
-- - Combos simultâneos (Shift + W pressionados ao mesmo tempo)
-- - Combos sequenciais (segurar Shift → depois apertar W)
-- - Sistema de cache para teclas modificadoras (Shift, Ctrl, Alt)
-- - Priorização inteligente de inputs
--
-- Exemplo de uso:
--
--   if IsKeyPressed("W") then
--		ExecEventAllBody("W_AttackRightHeavyCounter")
--		return
--	end
--
--
--   if IsComboPressed("Shift + W") then
--		ExecEventAllBody("W_AttackBothHeavyCounter")
--		return
--	end
--
--
-- // ===========================================================================

-- Map
local Key_Table = {
    [899000] = "A", [899001] = "B", [899002] = "C", [899003] = "D",
    [899004] = "E", [899005] = "F", [899006] = "G", [899007] = "H",
    [899008] = "I", [899009] = "J", [899010] = "K", [899011] = "L",
    [899012] = "M", [899013] = "N", [899014] = "O", [899015] = "P",
    [899016] = "Q", [899017] = "R", [899018] = "S", [899019] = "T",
    [899020] = "U", [899021] = "V", [899022] = "W", [899023] = "X",
    [899024] = "Y", [899025] = "Z",

    [899100] = "0", [899101] = "1", [899102] = "2", [899103] = "3",
    [899104] = "4", [899105] = "5", [899106] = "6", [899107] = "7",
    [899108] = "8", [899109] = "9",

    [899200] = "Left", [899201] = "Up", [899202] = "Right", [899203] = "Down",

    [899400] = "F1", [899401] = "F2", 	[899402] = "F3", 	[899403] = "F4",
    [899404] = "F5", [899405] = "F6", 	[899406] = "F7", 	[899407] = "F8",
    [899408] = "F9", [899409] = "F10", 	[899410] = "F11", 	[899411] = "F12",

    [899500] = "Space", 	[899501] = "Enter", [899502] = "Escape",
    [899503] = "Backspace", [899504] = "Tab", 	[899505] = "Shift",
    [899506] = "Ctrl", 		[899507] = "Alt",

    [899600] = ";", 	[899601] = "=", [899602] = ",", [899603] = "-",
    [899604] = ".", 	[899605] = "/", [899606] = "`", [899607] = "[",
    [899608] = "\\", 	[899609] = "]", [899610] = "'",

    -- GAMEPAD
    [899700] = "Gamepad_A",          -- A (Xbox) / Cross (PS)
    [899701] = "Gamepad_B",          -- B (Xbox) / Circle (PS)
    [899702] = "Gamepad_X",          -- X (Xbox) / Square (PS)
    [899703] = "Gamepad_Y",          -- Y (Xbox) / Triangle (PS)
	
    [899704] = "Gamepad_LB",         -- LB / L1
    [899705] = "Gamepad_RB",         -- RB / R1
    [899706] = "Gamepad_LT",         -- LT / L2 (botão)
    [899707] = "Gamepad_RT",         -- RT / R2 (botão)
	
    [899708] = "Gamepad_Back",       -- Back / Select
    [899709] = "Gamepad_Start",      -- Start
	
    [899710] = "Gamepad_L3",         -- L3 (pressionar stick esquerdo)
    [899711] = "Gamepad_R3",         -- R3 (pressionar stick direito)
	
    [899712] = "Gamepad_DPad_Up",    -- D-Pad Up
    [899713] = "Gamepad_DPad_Down",  -- D-Pad Down
    [899714] = "Gamepad_DPad_Left",  -- D-Pad Left
    [899715] = "Gamepad_DPad_Right", -- D-Pad Right
}

-- Cache
local KeyCache = {}				-- Armazena teclas
local CACHE_DURATION = 0.6 		-- Tempo em que uma tecla fica no cache

-- Functions

-- Verifica se uma tecla específica está atualmente pressionada
-- 单按键检测 例：if IsKeyPressed("H") then
function IsKeyPressed(keyName)
    for effectID, key in pairs(Key_Table) do
        if key == keyName and env(GetSpEffectID, effectID) == TRUE then
            return true
        end
    end
    return false
end

-- Verifica se uma combinação específica de teclas está pressionada simultaneamente
-- 收集所有按下的键：首先遍历 Key_Table，检查每个按键对应的 SpEffect 是否处于激活状态。所有被按下的键名（如 "Shift", "W", "Ctrl"）会被放进一个临时的 ActiveKeys 列表里。
-- 按字母顺序排序：使用 table.sort(ActiveKeys) 将列表中的所有键名按字母顺序重新排列。
-- 生成组合字符串：用 + 作为分隔符，将排序后的键名连接成一个单一的字符串。例如，如果 ActiveKeys 是 {"Shift", "W"}，它会生成字符串 "Shift + W"。
-- 进行比较：最后将这个生成的字符串与传入的 comboString 参数进行精确比较。如果完全相同，函数返回 true，否则返回 false。
-- 多按键检测 例：if IsComboPressed("Alt + Shift") then
function IsComboPressed(comboString)
    local ActiveKeys = {}

    for effectID, key in pairs(Key_Table) do
        if env(GetSpEffectID, effectID) == TRUE then
            table.insert(ActiveKeys, key)
        end
    end

    if #ActiveKeys > 0 then
        table.sort(ActiveKeys)
        local currentCombo = table.concat(ActiveKeys, " + ")
        --act(10001, "Current: " .. currentCombo .. " | Expected: " .. comboString)
        return currentCombo == comboString
    end
    
    return false
end

-- // ================================================================

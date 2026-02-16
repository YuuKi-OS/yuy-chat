# yuy-chat - Guía de Uso Completa

## 📖 Contenido

1. [Instalación](#instalación)
2. [Primera Vez](#primera-vez)
3. [Uso Diario](#uso-diario)
4. [Configuración Avanzada](#configuración-avanzada)
5. [Integración con HuggingFace](#integración-con-huggingface)
6. [Tips y Trucos](#tips-y-trucos)
7. [Troubleshooting](#troubleshooting)

---

## 🔧 Instalación

### Termux (Android)

```bash
# Instalar Rust
pkg install rust

# Clonar y compilar
git clone https://github.com/YuuKi-OS/yuy-chat
cd yuy-chat
cargo build --release -j 1  # Usar 1 thread en Termux

# Instalar globalmente
cargo install --path .
```

### Linux/macOS

```bash
# Clonar y compilar
git clone https://github.com/YuuKi-OS/yuy-chat
cd yuy-chat
cargo build --release

# Instalar
cargo install --path .
```

### Windows

```bash
# Mismo proceso que Linux/macOS
git clone https://github.com/YuuKi-OS/yuy-chat
cd yuy-chat
cargo build --release
cargo install --path .
```

---

## 🎬 Primera Vez

### 1. Asegúrate de tener modelos

yuy-chat busca modelos en `~/.yuuki/models/` por defecto.

**Opción A: Usar yuy**
```bash
yuy download Yuuki-best
```

**Opción B: Copiar manualmente**
```bash
mkdir -p ~/.yuuki/models/
cp /path/to/your/model.gguf ~/.yuuki/models/
```

### 2. Instalar llama.cpp

**Termux:**
```bash
pkg install llama-cpp
```

**macOS:**
```bash
brew install llama.cpp
```

**Linux:**
```bash
# Descargar desde releases
wget https://github.com/ggerganov/llama.cpp/releases/...
chmod +x llama-cli
sudo mv llama-cli /usr/local/bin/
```

### 3. Ejecutar yuy-chat

```bash
yuy-chat
```

Verás el selector de modelos. Usa `↑/↓` para navegar y `Enter` para seleccionar.

---

## 💬 Uso Diario

### Flujo Básico

```
1. Ejecuta: yuy-chat
   ↓
2. Selecciona modelo con ↑/↓ y Enter
   ↓
3. Escribe tu mensaje
   ↓
4. Presiona Enter para enviar
   ↓
5. Yuuki responde (streaming)
   ↓
6. Continúa la conversación
```

### Atajos de Teclado Útiles

**En chat:**
- `Enter` - Enviar mensaje
- `Shift+Enter` - Nueva línea (para mensajes multi-línea)
- `Ctrl+L` - Limpiar chat
- `Ctrl+S` - Guardar conversación
- `Ctrl+C` - Abrir menú

**Escribir código:**
```
You: Dame un ejemplo de código Python

[Shift+Enter para nueva línea]
def hello():
    print("Hola")
[Shift+Enter]

hello()

[Ctrl+Enter para enviar]
```

### Cambiar Preset

```
1. Ctrl+C (abrir menú)
   ↓
2. Presiona 2 (Change Preset)
   ↓
   Cicla entre: Creative → Balanced → Precise
```

**Cuándo usar cada preset:**
- **Creative**: Escribir historias, brainstorming, ideas
- **Balanced**: Uso general, conversación
- **Precise**: Código, matemáticas, datos exactos

---

## ⚙️ Configuración Avanzada

### Cambiar Directorio de Modelos

**Método 1: Configuración**
```bash
yuy-chat
Ctrl+C → 6 (Settings)
Editar "Models Directory"
```

**Método 2: Archivo config**
```bash
nano ~/.config/yuy-chat/config.toml
```

```toml
models_dir = "/custom/path/to/models"
```

### Personalizar Presets

Edita el código o usa parámetros de llama.cpp directamente:

```bash
# En models/runtime.rs, modifica:
pub fn temperature(&self) -> f32 {
    match self {
        Preset::Creative => 0.9,  // Más aleatorio
        // ...
    }
}
```

### Tema Claro

```toml
theme = "Light"
```

---

## 🌐 Integración con HuggingFace

### 1. Obtener Token

1. Ve a https://huggingface.co/settings/tokens
2. Click "Create new token"
3. Tipo: "Read"
4. Copia el token

### 2. Configurar en yuy-chat

**Método A: UI**
```
Ctrl+C → 6 (Settings)
Navigate to "HuggingFace Token"
Enter → Pega tu token
```

**Método B: Config file**
```toml
hf_token = "hf_abcdefghijklmnopqrstuvwxyz1234567890"
```

### 3. Usar Modelos de HF

Después de configurar el token:
```
yuy-chat
[Verás modelos locales + modelos HF API]

> Yuuki-best.gguf (Local)
  Yuuki-3.7.gguf (Local)  
  Yuuki-best (HF API) <-- Usa la API
```

**Ventajas:**
- No ocupa espacio local
- Siempre actualizado
- Acceso a modelos privados

**Desventajas:**
- Requiere internet
- Más lento que local
- Rate limits en plan gratis

---

## 💡 Tips y Trucos

### Guardar Conversaciones Importantes

```
Ctrl+S mientras chateas
→ Se guarda en ~/.config/yuy-chat/conversations/
```

### Cargar Conversación Anterior

```
Ctrl+C → 4 (Load Conversation)
↑/↓ para navegar
Enter para cargar
```

### Prompt Engineering

**Para mejores respuestas, sé específico:**

❌ Malo:
```
You: Explica Rust
```

✅ Bueno:
```
You: Explica el sistema de ownership en Rust con un ejemplo simple de borrowing. Quiero entender por qué evita memory leaks.
```

### Conversaciones Multi-paso

```
You: Vamos a diseñar una API REST

Yuuki: Claro, ¿qué tipo de API?

You: Para gestionar tareas tipo TODO

Yuuki: Perfecto, estos son los endpoints...
```

### Usar Presets Dinámicamente

- **Creative preset**: "Escribe un cuento de terror"
- **Precise preset**: "¿Cuál es la complejidad de quicksort?"
- **Balanced preset**: "Explícame cómo funciona Git"

---

## 🔧 Troubleshooting

### Error: "No models found"

**Solución:**
```bash
# Verifica que tienes modelos
ls ~/.yuuki/models/

# Si está vacío, descarga uno
yuy download Yuuki-best

# O especifica otro directorio
yuy-chat --models-dir /path/to/models
```

### Error: "llama.cpp binary not found"

**Solución:**
```bash
# Termux
pkg install llama-cpp

# macOS
brew install llama.cpp

# Linux - verifica que está en PATH
which llama-cli
# Si no, instala o agrega al PATH
export PATH=$PATH:/path/to/llama-cpp
```

### Error: "Permission denied" (llamafile)

**Solución:**
```bash
chmod +x ~/.yuuki/models/*.llamafile
```

### Chat no responde / se congela

**Diagnóstico:**
1. Verifica que llama.cpp funciona:
```bash
llama-cli -m ~/.yuuki/models/Yuuki-best.gguf -p "Hola"
```

2. Revisa logs:
```bash
RUST_LOG=debug yuy-chat
```

3. Reduce context size si es falta de RAM

### Respuestas muy lentas

**Causas comunes:**
- Modelo muy grande para tu RAM
- Cuantización muy alta (F32, Q8)
- Sin aceleración GPU

**Solución:**
```bash
# Descarga versión cuantizada más pequeña
yuy download Yuuki-best --quant q4_0

# Verifica RAM disponible
free -h  # Linux
top      # macOS/Linux
```

### No puedo escribir mensajes largos

El input box tiene límite visual pero **no de contenido**:
- Usa `Shift+Enter` para multi-línea
- Scroll automático después de 5 líneas
- O escribe en editor externo y pega

### HuggingFace API no funciona

**Verifica:**
```bash
# Test manual
curl https://api-inference.huggingface.co/models/OpceanAI/Yuuki-best \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{"inputs": "test"}'
```

**Problemas comunes:**
- Token expirado → Genera nuevo
- Rate limit → Espera o upgrade plan
- Modelo privado → Verifica permisos

---

## 📊 Performance Tips

### Termux/Móvil

```bash
# Usa modelos pequeños
yuy download Yuuki-best --quant q4_0

# Preset Balanced o Precise
# Creative es más lento
```

### Desktop High-end

```bash
# Usa Q8 o F32 para mejor calidad
yuy download Yuuki-best --quant q8_0

# Habilita GPU en llama.cpp
llama-cli -m model.gguf -ngl 32  # 32 layers en GPU
```

---

## 🎓 Casos de Uso

### 1. Coding Assistant

```
Preset: Precise

You: Cómo implemento un servidor HTTP en Rust?
You: Muestra ejemplo con tokio
You: Agrega manejo de errores
You: Ahora agrega logging
```

### 2. Creative Writing

```
Preset: Creative

You: Escribe el inicio de una novela de ciencia ficción ambientada en Marte en el año 2157
You: Continúa describiendo al protagonista
You: ¿Qué conflicto enfrenta?
```

### 3. Learning/Study

```
Preset: Balanced

You: Explícame la diferencia entre mutex y semaphore
You: Dame un ejemplo de cuándo usar cada uno
You: ¿Qué pasa si no uso sincronización?
```

---

## 🚀 Workflow Recomendado

### Developer

```bash
# Mañana: Coding
yuy-chat  # Preset: Precise
> Ayuda con bugs, arquitectura, código

# Tarde: Docs
yuy-chat  # Preset: Balanced
> Escribir documentación, READMEs

# Noche: Ideas
yuy-chat  # Preset: Creative
> Brainstorming features
```

### Writer

```bash
yuy-chat  # Preset: Creative
> Generar ideas
> Escribir borradores
> Feedback de historias
```

### Estudiante

```bash
yuy-chat  # Preset: Balanced
> Explicaciones de conceptos
> Resolver dudas
> Preparar exámenes
```

---

**¿Preguntas? Abre un issue en GitHub!**

🌸 Hecho con amor por el equipo Yuuki

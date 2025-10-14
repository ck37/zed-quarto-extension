#include <napi.h>

typedef struct TSLanguage TSLanguage;

extern "C" TSLanguage *tree_sitter_quarto();

namespace {

Napi::Object Init(Napi::Env env, Napi::Object exports) {
  exports["name"] = Napi::String::New(env, "quarto");
  auto language = Napi::External<TSLanguage>::New(env, tree_sitter_quarto());
  exports["language"] = language;
  return exports;
}

NODE_API_MODULE(tree_sitter_quarto_binding, Init)

}  // namespace

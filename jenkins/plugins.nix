# Jenkins plugin definitions — configuration-as-code + workflow-aggregator
# and all of their required transitive dependencies.
#
# To regenerate this file (e.g. after adding a plugin or updating versions):
#
#   python3 jenkins/generate-plugins.py > jenkins/plugins.nix
#
# The generator queries the Jenkins stable update center, resolves transitive
# dependencies, fetches each .hpi, and writes this file with correct hashes.
# Adding a new top-level plugin means editing the `wanted` list at the top of
# generate-plugins.py and re-running it.
{ fetchurl, stdenv }:
{
  "caffeine-api" = stdenv.mkDerivation {
    name = "caffeine-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/caffeine-api/3.2.3-194.v31a_b_f7a_b_5a_81/caffeine-api.hpi";
      hash = "sha256-k+IYY36Zq3lO/mZjhxla8PErG9e+81x+QOkewM9sQ+8=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "commons-lang3-api" = stdenv.mkDerivation {
    name = "commons-lang3-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/commons-lang3-api/3.20.0-109.ve43756e2d2b_4/commons-lang3-api.hpi";
      hash = "sha256-qgkI4AAsI8Mp3x9eNtLqmvbfvXx/cfz0E8xHHzNcIFI=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "commons-text-api" = stdenv.mkDerivation {
    name = "commons-text-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/commons-text-api/1.15.0-218.va_61573470393/commons-text-api.hpi";
      hash = "sha256-riq9+VdpT0qjm5ugtTntq8OE89ondVGHnjT/si+xN2Q=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "json-api" = stdenv.mkDerivation {
    name = "json-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/json-api/20251224-185.v0cc18490c62c/json-api.hpi";
      hash = "sha256-kXCkxVzZSeV7+JcFcEGIaqqzoaWbqKOhhC8JTGg9Tis=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "structs" = stdenv.mkDerivation {
    name = "structs";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/structs/362.va_b_695ef4fdf9/structs.hpi";
      hash = "sha256-oKXbRPLLCCqexlcsCINAUh8DP/+MNR/H66nvfwfbv40=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "workflow-step-api" = stdenv.mkDerivation {
    name = "workflow-step-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-step-api/710.v3e456cc85233/workflow-step-api.hpi";
      hash = "sha256-61FOFb/mz7l/dTUM+7vysJdcwZ/VUJysll02ys046Kg=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "asm-api" = stdenv.mkDerivation {
    name = "asm-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/asm-api/9.9.1-189.vb_5ef2964da_91/asm-api.hpi";
      hash = "sha256-FR3QTQ6KCRFITMy7MIfhieq0uxzrWo7DLblswclyLzo=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "scm-api" = stdenv.mkDerivation {
    name = "scm-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/scm-api/728.vc30dcf7a_0df5/scm-api.hpi";
      hash = "sha256-EzZHKYfaovqRIhCroA4eN7c5zgP3XJc6Z5TRoPVcAyY=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "workflow-api" = stdenv.mkDerivation {
    name = "workflow-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-api/1398.v67030756d3fb_/workflow-api.hpi";
      hash = "sha256-uZV3ErViPjcy6wQPKQHfMOLtHwHFPodyshnp7QPClPs=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "script-security" = stdenv.mkDerivation {
    name = "script-security";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/script-security/1399.ve6a_66547f6e1/script-security.hpi";
      hash = "sha256-g6KXmzAl9qfjBzTptOYvadUMGbimWab63vckJygCn/M=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "workflow-support" = stdenv.mkDerivation {
    name = "workflow-support";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-support/1015.v785e5a_b_b_8b_22/workflow-support.hpi";
      hash = "sha256-kU7nF6uMHWDQ0+pJoa8R5OplwedMC0TcNSL5ov5bKo4=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "plugin-util-api" = stdenv.mkDerivation {
    name = "plugin-util-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/plugin-util-api/6.1192.v30fe6e2837ff/plugin-util-api.hpi";
      hash = "sha256-0OaHhXp4p31uAjhE1dTwbD3zdRKhds9PZG8K6jpJjn4=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "font-awesome-api" = stdenv.mkDerivation {
    name = "font-awesome-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/font-awesome-api/7.2.0-965.ve3840b_696418/font-awesome-api.hpi";
      hash = "sha256-ZPdpNh31Rth4Nj5wWoVRccseeb2FMg8AnJQTj68otrw=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "bootstrap5-api" = stdenv.mkDerivation {
    name = "bootstrap5-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/bootstrap5-api/5.3.8-895.v4d0d8e47fea_d/bootstrap5-api.hpi";
      hash = "sha256-ax69jcHEEHpx5/VS+upv81ATBN+QpTB2+0xEt7dhecc=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "antisamy-markup-formatter" = stdenv.mkDerivation {
    name = "antisamy-markup-formatter";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/antisamy-markup-formatter/173.v680e3a_b_69ff3/antisamy-markup-formatter.hpi";
      hash = "sha256-KbR2V5fybERXSicy3fnX2Y6gKDKVNSSX3Z7vhK7QEC8=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "prism-api" = stdenv.mkDerivation {
    name = "prism-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/prism-api/1.30.0-703.v116fb_3b_5b_b_a_a_/prism-api.hpi";
      hash = "sha256-k/1Ou3YdUlhjw7CLUnTH5Yt/lMQYRvRyU9GWmhLtHLs=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "snakeyaml-api" = stdenv.mkDerivation {
    name = "snakeyaml-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/snakeyaml-api/2.5-149.v72471e9c6371/snakeyaml-api.hpi";
      hash = "sha256-gO2+7BW5r7yhjQAM82fZ4mRWqoMQDc3QnuDtakuYqcI=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "configuration-as-code" = stdenv.mkDerivation {
    name = "configuration-as-code";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/configuration-as-code/2053.vb_0da_47381a_25/configuration-as-code.hpi";
      hash = "sha256-A6YX7HpHkW3/8ZnIz2hrKBSuZ0Z3F5JlK+jNBjWHdAA=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "ionicons-api" = stdenv.mkDerivation {
    name = "ionicons-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/ionicons-api/94.vcc3065403257/ionicons-api.hpi";
      hash = "sha256-lfNQQ3XWKNiHQlzN8y1WBqIwJh7kLcaD8TlY2OlzrD8=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "workflow-scm-step" = stdenv.mkDerivation {
    name = "workflow-scm-step";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-scm-step/466.va_d69e602552b_/workflow-scm-step.hpi";
      hash = "sha256-jzW2gVPuimcdAdzcPYbdR4N70tX8+d+NZ0/4ruz7oWo=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "workflow-cps" = stdenv.mkDerivation {
    name = "workflow-cps";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-cps/4259.vf653c2b_8a_b_69/workflow-cps.hpi";
      hash = "sha256-cDfHCmrWiLVSg+wVSknv+arPmDI2Y4nbMKpRhOXYkmw=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "cloudbees-folder" = stdenv.mkDerivation {
    name = "cloudbees-folder";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/cloudbees-folder/6.1079.vc0975c2de294/cloudbees-folder.hpi";
      hash = "sha256-ZD1BobOpxZDsWha2LEOhBzwCqfaU2tyxH/ZID9SvHd8=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "variant" = stdenv.mkDerivation {
    name = "variant";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/variant/70.va_d9f17f859e0/variant.hpi";
      hash = "sha256-EuIU6mlEaaRGG1WIHstAdK5d+wR5fbwNrTkPb0vHWq8=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "pipeline-groovy-lib" = stdenv.mkDerivation {
    name = "pipeline-groovy-lib";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-groovy-lib/787.ve2fef0efdca_6/pipeline-groovy-lib.hpi";
      hash = "sha256-sWYmTgv5ut8hpe2LsdHjfSmFZQatQZyNm+IVxcmcY5U=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "jakarta-activation-api" = stdenv.mkDerivation {
    name = "jakarta-activation-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/jakarta-activation-api/2.1.4-1/jakarta-activation-api.hpi";
      hash = "sha256-5iKYNBdC8Us3QEU4e6ZBrYoZN2eTx2va/rtKBbUJlxI=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "jakarta-mail-api" = stdenv.mkDerivation {
    name = "jakarta-mail-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/jakarta-mail-api/2.1.5-1/jakarta-mail-api.hpi";
      hash = "sha256-JEc16oWlEzLAedI1pXE+cNOBC+piMyiUACrMOZFM4Rs=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "durable-task" = stdenv.mkDerivation {
    name = "durable-task";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/durable-task/664.v2b_e7a_dfff66c/durable-task.hpi";
      hash = "sha256-JRVR5ekbycBfhFvMxkbVINMX4Nh27wRofv/O0Jozp/M=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "workflow-durable-task-step" = stdenv.mkDerivation {
    name = "workflow-durable-task-step";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-durable-task-step/1464.v2d3f5c68f84c/workflow-durable-task-step.hpi";
      hash = "sha256-ACorw0nzZRIEdP77ir3wz47c83d1EgL9575lEIKH5yE=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "apache-httpcomponents-client-4-api" = stdenv.mkDerivation {
    name = "apache-httpcomponents-client-4-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/apache-httpcomponents-client-4-api/4.5.14-269.vfa_2321039a_83/apache-httpcomponents-client-4-api.hpi";
      hash = "sha256-zqL2HMcokLli/Ut2fDF3jGyzABW0qHWLTCa2LlHKlTM=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "bouncycastle-api" = stdenv.mkDerivation {
    name = "bouncycastle-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/bouncycastle-api/2.30.1.83-289.v8426fcd19371/bouncycastle-api.hpi";
      hash = "sha256-+4yywbwcCoC55YYuGhtI87jsB1rClHy0NDttydP46mM=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "instance-identity" = stdenv.mkDerivation {
    name = "instance-identity";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/instance-identity/203.v15e81a_1b_7a_38/instance-identity.hpi";
      hash = "sha256-3ysyBaYXckjxKylGUKYEBfZbos1QPKlZmy3LcsHeUBg=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "display-url-api" = stdenv.mkDerivation {
    name = "display-url-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/display-url-api/2.217.va_6b_de84cc74b_/display-url-api.hpi";
      hash = "sha256-B+6q5zi0UbreR1aQC0hWHGxJOkLHugekWyKi3OcawMo=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "mailer" = stdenv.mkDerivation {
    name = "mailer";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/mailer/525.v2458b_d8a_1a_71/mailer.hpi";
      hash = "sha256-hpK+jKRClX3PP2iffUEwMf43TyslNNqtiYdBmyksZuE=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "workflow-basic-steps" = stdenv.mkDerivation {
    name = "workflow-basic-steps";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-basic-steps/1098.v808b_fd7f8cf4/workflow-basic-steps.hpi";
      hash = "sha256-k0SetJRljQZdCX6MVPcHyYyc4+VNovuYKBsKBIcVTq8=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "workflow-job" = stdenv.mkDerivation {
    name = "workflow-job";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-job/1571.vb_423c255d6d9/workflow-job.hpi";
      hash = "sha256-Z0LBw3+7d16cPMaOxNgeso6rStZ5lUzFkAqU1YytHt0=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "branch-api" = stdenv.mkDerivation {
    name = "branch-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/branch-api/2.1280.v0d4e5b_b_460ef/branch-api.hpi";
      hash = "sha256-+pwkZkThtQESQaLm2t7QWsUOWfa5lBhxoBKznv6cjes=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "workflow-multibranch" = stdenv.mkDerivation {
    name = "workflow-multibranch";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-multibranch/821.vc3b_4ea_780798/workflow-multibranch.hpi";
      hash = "sha256-8KmFfgsHxSzFTkJ7btyidGEuERyrlBbS48PXaoPwDP0=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "pipeline-build-step" = stdenv.mkDerivation {
    name = "pipeline-build-step";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-build-step/584.vdb_a_2cc3a_d07a_/pipeline-build-step.hpi";
      hash = "sha256-iN8XkqyQGK3hmTtr+2t4LVcY8TAHZoWiVpIpGmURdB8=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "credentials" = stdenv.mkDerivation {
    name = "credentials";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/credentials/1491.v6d6145e96e1c/credentials.hpi";
      hash = "sha256-b8F8+3p20ptkBLtP8k0jcM1lbybqTDcPZbQq8PpNTtg=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "pipeline-input-step" = stdenv.mkDerivation {
    name = "pipeline-input-step";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-input-step/540.v14b_100d754dd/pipeline-input-step.hpi";
      hash = "sha256-NHbwz8ghn2x2qikb4jOjRoUDAk/+3XcAkfzBEB3R81c=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "pipeline-milestone-step" = stdenv.mkDerivation {
    name = "pipeline-milestone-step";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-milestone-step/138.v78ca_76831a_43/pipeline-milestone-step.hpi";
      hash = "sha256-B09/s3piUi3xA4z0NkPobRhb3KQ1I4XmeHma9NtbxNE=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "pipeline-stage-step" = stdenv.mkDerivation {
    name = "pipeline-stage-step";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-stage-step/322.vecffa_99f371c/pipeline-stage-step.hpi";
      hash = "sha256-5O1kOP9gcxVrxCadNiEps0CZoNp/Krsnmy4559wKdco=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "plain-credentials" = stdenv.mkDerivation {
    name = "plain-credentials";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/plain-credentials/199.v9f8e1f741799/plain-credentials.hpi";
      hash = "sha256-teXjXPk607gYqYORbLzfP3hmaZQQpN+SPmopaOzDpEg=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "ssh-credentials" = stdenv.mkDerivation {
    name = "ssh-credentials";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/ssh-credentials/361.vb_f6760818e8c/ssh-credentials.hpi";
      hash = "sha256-HoDHVP/94JfguxrENsRON3rKKB6xEzoi3soF3YRTNFo=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "credentials-binding" = stdenv.mkDerivation {
    name = "credentials-binding";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/credentials-binding/717.v951d49b_5f3a_a_/credentials-binding.hpi";
      hash = "sha256-7keN3UaejJHJZHNbHcZ6qeALBf4ODYe1F5rhHEdf+Es=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "joda-time-api" = stdenv.mkDerivation {
    name = "joda-time-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/joda-time-api/2.14.1-187.vdf2def02b_8a_1/joda-time-api.hpi";
      hash = "sha256-WJ+gx2w5Xc+YVDzMQgCq9aGpBONo4cUyQ586XJyYJck=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "jakarta-xml-bind-api" = stdenv.mkDerivation {
    name = "jakarta-xml-bind-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/jakarta-xml-bind-api/4.0.6-12.vb_1833c1231d3/jakarta-xml-bind-api.hpi";
      hash = "sha256-kw0ee9wjpkbSiXOhOBUfynFDFLHsuj4iBBW5YJ2gqeY=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "javax-activation-api" = stdenv.mkDerivation {
    name = "javax-activation-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/javax-activation-api/1.2.0-8/javax-activation-api.hpi";
      hash = "sha256-6W6IxS7fB7oA+0WybMQRp6lfo9RJGqoaXTZjdmWIBWA=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "jaxb" = stdenv.mkDerivation {
    name = "jaxb";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/jaxb/2.3.9-143.v5979df3304e6/jaxb.hpi";
      hash = "sha256-ON30z122GlkQRBMML3zLK3qpq+vI/NimlT+NpVDQAu0=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "jackson2-api" = stdenv.mkDerivation {
    name = "jackson2-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/jackson2-api/2.21.1-428.vf8dd988fa_d8d/jackson2-api.hpi";
      hash = "sha256-YxwDvYRaXEvEsrrCdsdz0/Ew8Nb4Xr//AvghG8rRd5I=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "pipeline-model-api" = stdenv.mkDerivation {
    name = "pipeline-model-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-model-api/2.2277.v00573e73ddf1/pipeline-model-api.hpi";
      hash = "sha256-r+2881NlJv8462QPctJ9joKLUOYe0tRNtmAvhc26sFQ=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "pipeline-model-extensions" = stdenv.mkDerivation {
    name = "pipeline-model-extensions";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-model-extensions/2.2277.v00573e73ddf1/pipeline-model-extensions.hpi";
      hash = "sha256-SDE9iMbUAyIcyp7m5LIrfKoiYMuzCuGhMhUv34a20dI=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "pipeline-stage-tags-metadata" = stdenv.mkDerivation {
    name = "pipeline-stage-tags-metadata";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-stage-tags-metadata/2.2277.v00573e73ddf1/pipeline-stage-tags-metadata.hpi";
      hash = "sha256-tly1rLXn8VwBwmd57gFYW/3C0u8uhsOOtmErvnZIKmY=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "pipeline-model-definition" = stdenv.mkDerivation {
    name = "pipeline-model-definition";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-model-definition/2.2277.v00573e73ddf1/pipeline-model-definition.hpi";
      hash = "sha256-FO4YhzDE1LqDJZr2H1hQNtXDTtC526RtMBK+mA81EsI=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
  "workflow-aggregator" = stdenv.mkDerivation {
    name = "workflow-aggregator";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-aggregator/608.v67378e9d3db_1/workflow-aggregator.hpi";
      hash = "sha256-e5QifBr6AbsmLBSRmtSjM0EvG1HZR8DyGQnsLa/zV9w=";
    };
    phases = [ "installPhase" ];
    installPhase = "cp $src $out";
  };
}
